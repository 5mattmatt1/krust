use crate::{serial_println, asm};
use heapless;
use heapless::consts::*;

/* Gotten from https://wiki.osdev.org/RTL8139 */
const MAC05_OFFSET: u32 = 0x00;
const MAR07_OFFSET: u32 = 0x08;
const RBSTART_OFFSET: u32 = 0x30;
const CMD_OFFSET: u32 = 0x37;
const IMR_OFFSET: u32 = 0x3C;
const ISR_OFFSET: u32 = 0x3E;
const CONFIG_1_REG: u32 = 0x52;
const RCR_OFFSET: u32 = 0x44;

/* Gotten from drivers/net/ethernet/realtek */
#[repr(u8)]
pub enum RTL8139Registers {
    MAC0 = 0x0,
    MAR0 = 0x8,
    TxStatus0 = 0x10,
    TxAddr0 = 0x20,
    RxBuf = 0x30,
    ChipCmd = 0x37,
    RxBufPtr = 0x38,
    RxBufAddr = 0x3A,
    IntrMask = 0x3C,
    IntrStatus = 0x3E,
    TxConfig = 0x40,
    RxConfig = 0x44,
    Timer = 0x48,
    RxMissed = 0x4C,
    Cfg9346 = 0x50,
    Config0 = 0x51,
    Config1 = 0x52,
    TimerInt = 0x54,
    MediaStatus = 0x58,
    Config3 = 0x59,
    Config4 = 0x5A,
    HltClk = 0x5B,
    MultiIntr = 0x5C,
    TxSummary = 0x60,
    BasicModeCtrl = 0x62,
    BasicModeStatus = 0x64,
    NWayAdvert = 0x66,
    NWayLPAR = 0x68,
    NWayExpansion = 0x6A,
}
/* Undocumented registers */
// I don't trust these 
// const FIFOTMS: u32 = 0x70;
// const CSCR: u32 = 0x74;
// const PARA78: u32 = 0x78;
// const FlashReg: u32 = 0xD4;
// const PARA7c: u32 = 0x7C;
// const Config5: u32 = 0xD8;

// ChipCmdBits
#[repr(u8)]
pub enum ChipCmdBits
{
    CmdReset = 0x10,
    CmdRxEnb = 0x08,
    CmdTxEnb = 0x04,
    RxBufEmpty= 0x01,
}
// IntrStatusBits
#[repr(u16)]
pub enum IntrStatusBits
{
    PCIErr = 0x8000,
    PCSTimeout = 0x4000,
    RxFIFOOver = 0x40,
    RxUnderrun = 0x20,
    RxOverflow = 0x10,
    TxErr = 0x08,
    TxOK = 0x04,
    RxErr = 0x02,
    RxOK = 0x01,
    // RxAckBits = RxFIFOOver | RxOverflow | RxOk,
}

// TxStatusBits
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum TxStatusBits
{
    TxHostOwns = 0x2000,
    TxUnderrun = 0x4000,
    TxStatOk = 0x8000,
    TxOutOfWindow = 0x20000000,
    TxAborted = 0x40000000,
    TxCarrierLost = 0x80000000,
    TxInvalidStatusBit = 0,
}

pub fn u32_to_tx_status_bits(input: u32) -> TxStatusBits
{
    use TxStatusBits::*;
    let uTxHostOwns: u32 = TxHostOwns as u32;
    let uTxUnderrun: u32 = TxUnderrun as u32;
    match input
    {
        uTxHostOwns => return TxHostOwns,
        uTxUnderrun => return TxUnderrun,
        _ => return TxInvalidStatusBit,
    }
}

// RxStatusBits
#[repr(u16)]
pub enum RxStatusBits
{
    RxMulticast = 0x8000,
    RxPhysical = 0x4000,
    RxBroadcast = 0x2000,
    RxBadSymbol = 0x0020,
    RxRunt = 0x0010,
    RxTooLong = 0x0008,
    RxCRCErr	= 0x0004,
	RxBadAlign	= 0x0002,
	RxStatusOK	= 0x0001,
}

#[repr(u8)]
pub enum RxModeBits
{
    AcceptErr	= 0x20,
	AcceptRunt	= 0x10,
	AcceptBroadcast	= 0x08,
	AcceptMulticast	= 0x04,
	AcceptMyPhys	= 0x02,
	AcceptAllPhys	= 0x01,
}

#[repr(u32)]
pub enum TxConfigBits
{
    /* Interframe Gap Time. Only TxIFG96 doesn't violate IEEE 802.3 */
    // TxIFGShift	= 24,
    TxIFG84		= 0x1000000, /* 8.4us / 840ns (10 / 100Mbps) (0 << TxIFGShift) */
    TxIFG88		= 0x2000000, /* 8.8us / 880ns (10 / 100Mbps) (1 << TxIFGShift)*/
    TxIFG92		= 0x3000000, /* 9.2us / 920ns (10 / 100Mbps) (2 << TxIFGShift)*/
    TxIFG96		= 0x4000000, /* 9.6us / 960ns (10 / 100Mbps) (3 << TxIFGShift)*/

	TxLoopBack	= 0x60000, /* enable loopback test mode */
	TxCRC		= 0x10000,	/* DISABLE Tx pkt CRC append */
	TxClearAbt	= 0x1,	/* Clear abort (WO) */
	TxDMAShift	= 0x8, /* DMA burst value (0-7) is shifted X many bits */
	TxRetryShift= 0x4, /* TXRR value (0-15) is shifted X many bits */

	TxVersionMask = 0x7C800000, /* mask out version bits 30-26, 23 */
}

#[repr(u32)]
pub enum RxConfigBits
{
    /* rx fifo threshold */
	// RxCfgFIFOShift	= 13,
	RxCfgFIFONone	= 0xE000,

	/* Max DMA burst */
	// RxCfgDMAShift	= 8,
	RxCfgDMAUnlimited = 0x700,

	/* rx ring buffer length */
	RxCfgRcv8K	= 0,
	RxCfgRcv16K	= 0x800,
	RxCfgRcv32K	= 0x1000,
	RxCfgRcv64K	= 0x1800,

	/* Disable packet wrap at end of Rx buffer. (not possible with 64k) */
	RxNoWrap	= 0x80,
}

#[repr(u8)]
pub enum Cfg9346Bits
{
    Cfg9346_Lock	= 0x00,
	Cfg9346_Unlock	= 0xC0,
}

const RTL8139_INTR_MASK: u16 =
	IntrStatusBits::PCIErr as u16 | IntrStatusBits::PCSTimeout as u16 | IntrStatusBits::RxUnderrun as u16 | 
    IntrStatusBits::RxOverflow as u16 | IntrStatusBits::RxFIFOOver as u16 | IntrStatusBits::TxErr as u16 | 
    IntrStatusBits::TxOK as u16 | IntrStatusBits::RxErr as u16 | IntrStatusBits::RxOK as u16;
/* Sizes of registers */
/*
const MAC05_SIZE: u8 = 0x06;
const MAR07_SIZE: u8 = 0x08;
const RBSTART_SIZE: u8 = 0x04;
const CMD_SIZE: u8 = 1;
const IMR_SIZE: u8 = 0x02;
const ISR_SIZE: u8 = 0x02;
*/
const NUM_TX_DESC: u8 = 4;

// Note:
// bus = 0
// slot = 3

/*
// Should check to see if this is a CONFIG_NET_POLL_CONTROLLER
struct net_device_ops rtl8139_netdev_ops = {
    .ndo_open		= rtl8139_open,
	.ndo_stop		= rtl8139_close,
	.ndo_get_stats64	= rtl8139_get_stats64,
	.ndo_validate_addr	= eth_validate_addr, /* Probably in ethtools.h */
	.ndo_set_mac_address 	= rtl8139_set_mac_address,
	.ndo_start_xmit		= rtl8139_start_xmit,
	.ndo_set_rx_mode	= rtl8139_set_rx_mode,
	.ndo_do_ioctl		= netdev_ioctl,
	.ndo_tx_timeout		= rtl8139_tx_timeout,
#ifdef CONFIG_NET_POLL_CONTROLLER
	.ndo_poll_controller	= rtl8139_poll_controller,
#endif
	.ndo_set_features	= rtl8139_set_features,
}

static const struct ethtool_ops rtl8139_ethtool_ops = {
	.get_drvinfo		= rtl8139_get_drvinfo,
	.get_regs_len		= rtl8139_get_regs_len,
	.get_regs		= rtl8139_get_regs,
	.nway_reset		= rtl8139_nway_reset,
	.get_link		= rtl8139_get_link,
	.get_msglevel		= rtl8139_get_msglevel,
	.set_msglevel		= rtl8139_set_msglevel,
	.get_wol		= rtl8139_get_wol,
	.set_wol		= rtl8139_set_wol,
	.get_strings		= rtl8139_get_strings,
	.get_sset_count		= rtl8139_get_sset_count,
	.get_ethtool_stats	= rtl8139_get_ethtool_stats,
	.get_link_ksettings	= rtl8139_get_link_ksettings,
	.set_link_ksettings	= rtl8139_set_link_ksettings,
};
*/

/* 
// To be properly implemented
trait NetDeviceOps {
    fn ndo_open();
    fn ndo_stop();
    fn ndo_get_stats64();
    fn ndo_validate_addr();
    fn ndo_set_mac_address();
    fn ndo_start_xmit();
    fn ndo_set_rx_mode();
    fn ndo_do_ioctl();
    fn ndo_tx_timeout();
    fn ndo_poll_controller();
    fn ndo_set_features();
}

trait EthtoolOps {
    fn get_drvinfo();
    fn get_regs_len();
    fn get_regs();
    fn nway_rest();
    fn get_link();
    fn set_msglevel();
    fn get_wol();
    fn set_wol();
    fn get_strings();
    fn get_sset_count();
    fn get_ethtool_stats();
    fn get_link_ksettings();
    fn set_link_ksettings();
}
*/

/* Going to make some of these nonpublic */
/*
pub unsafe fn power_on(ioaddr: u32)
{
    let power_on_address: u32 = ioaddr+CONFIG_1_REG;
    // asm!("outl %eax, %dx" :: "{dx}"(power_on_address), "{eax}"(0x0) :: "volatile");
    /* RTL_W8(Config1, ~(SLEEP | PWRDN)) */
    asm::outl(power_on_address, 0x0);
}

pub unsafe fn init_rx(ioaddr: u32, rxbuffer_addr: &[u8; 1024])
{
    let rbstart_address: u32 = ioaddr + RBSTART_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(rbstart_address), "{eax}"(rxbuffer_addr) :: "volatile");
    // asm::outl(rxbuffer_addr, rbstart_address);
}

pub unsafe fn accept_tok(ioaddr: u32)
{
    let imroffset_address: u32 = ioaddr + IMR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(imroffset_address), "{eax}"(0x5) :: "volatile");
    // asm::outl(0x5, imroffset_address);
}

pub unsafe fn accept_rok(ioaddr: u32)
{
    let isroffset_address: u32 = ioaddr + ISR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(isroffset_address), "{eax}"(0x5) :: "volatile");
    // asm::outl(0x5, isroffset_address);
}

pub unsafe fn config_recieve_buffer_prom(ioaddr: u32)
{
    // outportl(ioaddr + 0x44, 0xf | (1 << 7));
    // Not configuring wrap bit
    let rcr_address: u32 = ioaddr + RCR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(rcr_address), "{eax}"(0xf) :: "volatile");
    // asm::outl(0x44, rcr_address);
}


pub unsafe fn enable_recieve_and_transmit(ioaddr: u32)
{
    let rete_address: u32 = ioaddr + CMD_OFFSET;
    // outportb(ioaddr + CMD_OFFSET, 0x0C); // Sets the RE and TE bits high
    asm!("outl %eax, %dx" :: "{dx}"(rete_address), "{eax}"(0x10) :: "volatile");
    // asm::outl(0x0c, rete_address);
}
pub unsafe fn acknowledge_rok(ioaddr: u32)
{
    let ackaddr = ioaddr + ISR_OFFSET;
    // outportw(ioaddr + ISR_OFFSET, 0x1);
    asm!("outl %eax, %dx" :: "{dx}"(ackaddr), "{eax}"(0x10) :: "volatile");
}
*/

/* Can implement sg/csum/highdma through skb_copy_and_csum_dev */
/*
pub unsafe fn setup_rtl8139(ioaddr: u32, rxbuffer_addr: &[u8; 1024]) -> bool
{
    /* Check into alloc_etherdev might be in ethtools.h or whatever */
    /* Check into pci_enable_device */
    /* Check into pci_resource_len */
    let mut success: bool = true;
    power_on(ioaddr);
    success &= chip_reset(ioaddr);
    init_rx(ioaddr, rxbuffer_addr);
    accept_tok(ioaddr);
    accept_rok(ioaddr);
    config_recieve_buffer_prom(ioaddr);
    enable_recieve_and_transmit(ioaddr);

    return success;
}
*/

/* Line: 714
static const unsigned int rtl8139_tx_config =
	TxIFG96 | (TX_DMA_BURST << TxDMAShift) | (TX_RETRY << TxRetryShift);
*/

pub struct RTL8139Driver
{
    bus: u8,
    slot: u8, /* Really u4 */
    cur_tx: u32,
    dirty_tx: u32,
    ioaddr: u32,
}

// Definitely need to move to a struct system
impl RTL8139Driver
{
    #[inline]
    pub unsafe fn RTL_R8(&self, offset: RTL8139Registers) -> u8
    {
        let retVal: u8;
        retVal = asm::inb(self.ioaddr + offset as u32);
        return retVal;
    }

    #[inline]
    pub unsafe fn RTL_W8(&self, offset: RTL8139Registers, value: u8)
    {
        asm::outb(self.ioaddr + offset as u32, value);
    }

    #[inline]
    pub unsafe fn RTL_R16(&self, offset: RTL8139Registers) -> u16
    {
        let retVal: u16;
        retVal = asm::inw(self.ioaddr + offset as u32);
        return retVal;
    }

    #[inline]
    pub unsafe fn RTL_W16(&self, offset: RTL8139Registers, value: u16)
    {
        asm::outw(self.ioaddr + offset as u32, value);
    }

    #[inline]
    pub unsafe fn RTL_R32(&self, offset: RTL8139Registers) -> u32
    {
        let retVal: u32;
        retVal = asm::inl(self.ioaddr + offset as u32);
        return retVal;
    }

    #[inline]
    pub unsafe fn RTL_RU32(&self, offset: u32) -> u32
    {
        let retVal: u32;
        retVal = asm::inl(self.ioaddr + offset);
        return retVal;
    }

    #[inline]
    pub unsafe fn RTL_W32(&self, offset: RTL8139Registers, value: u32)
    {
        asm::outl(self.ioaddr + offset as u32, value);
    }

    pub fn new(bus: u8, slot: u8) -> RTL8139Driver
    {
        use crate::pci::pci_slconf1_read;
        let baseio_address: u32 = unsafe { pci_slconf1_read(bus, slot, 0, 0x10) };
        RTL8139Driver {
            bus: bus, 
            slot: slot, 
            cur_tx: 0, 
            dirty_tx: 0, 
            ioaddr: baseio_address
        }
    }

    pub unsafe fn chip_reset(&self) -> bool
    {
        use RTL8139Registers::{ChipCmd};
        use ChipCmdBits::{CmdReset};
        self.RTL_W8(ChipCmd, CmdReset as u8);
        /* Possibly add for loop to check if it resets */
        return ((self.RTL_R8(ChipCmd)) & CmdReset as u8) == 0;
    }

    /*
    unsafe fn init_ring()
    {
        // self.cur_rx = 0;
        // self.cur_tx = 0;
        // self.dirty_tx = 0;
        for i in 0..NUM_TX_DESC
        {
            self.tx_buf[i] = self.tx_bufs[i * TX_BUF_SIZE];
        }
    }
    */

    pub unsafe fn hw_start(&self)
    {
        use RTL8139Registers::{ChipCmd, RxConfig, RxMissed, Cfg9346, IntrMask};
        use ChipCmdBits::{CmdRxEnb, CmdTxEnb};
        use RxModeBits::{AcceptBroadcast, AcceptMyPhys};
        use Cfg9346Bits::{Cfg9346_Unlock, Cfg9346_Lock};
        // Might need to bring out of low power mode since it is an old chip
        self.chip_reset();
        // RTL_W8(Cfg9346, Cfg9346Unlock)
        // might want to use outb instead
        // asm!("outl %eax, %dx" :: "{dx}"(Cfg9346), "{eax}"(Cfg9346Unlock) :: "volatile");
        self.RTL_W32(Cfg9346, Cfg9346_Unlock as u32);
        /* Restore our idea of the MAC address */
        // Line 1395
        // Line 1396

        /* How to get rx buffer DMA address? */
        // Line 1401

        // Need to implement RTL_W_F, and RTL_R_F
        // Better but still need to implement those sweet sweet macros
        // Need to test if #inline stops the registers from being clobbered
        // asm!("outl %eax, %dx" :: "{dx}"(ioaddr + ChipCmd as u32), "{eax}"(CmdRxEnb as u32 | CmdTxEnb as u32) :: "volatile");
        self.RTL_W32(ChipCmd, CmdRxEnb as u32 | CmdTxEnb as u32);
        // asm!("outl %eax, %dx" :: "{dx}"(ioaddr + RxConfig as u32), "{eax}"(AcceptBroadcast as u32 | AcceptMyPhys as u32) :: "volatile")
        self.RTL_W32(RxConfig, AcceptBroadcast as u32 | AcceptMyPhys as u32);
        // RTL_W32(ioaddr, TxConfig, rtl8139_tx_config);

        /* Relock Cfg9346 */
        self.RTL_W32(Cfg9346, Cfg9346_Lock as u32);

        self.RTL_W32(RxMissed, 0);
        // set_rx_mode()

        /* no early-rx interrupts */
        /* make sure RxTx has started */
        /*
        tmp = RTL_R8 (ChipCmd);
        if ((!(tmp & CmdRxEnb)) || (!(tmp & CmdTxEnb)))
            RTL_W8 (ChipCmd, CmdRxEnb | CmdTxEnb);
        // By the time you read the register, do an if statement, do fancy bit masking as not gates, is it not worth it to just write a second time?
        */

        /* Enable all known interrupts by setting the interrupt mask */
        self.RTL_W16(IntrMask, RTL8139_INTR_MASK);
    }

    unsafe fn tx_interrupt(&mut self)
    {
        use RTL8139Registers::{TxStatus0, TxConfig, IntrStatus};
        use TxStatusBits::{TxStatOk, TxUnderrun, TxAborted};
        use IntrStatusBits::{TxErr};
        use TxConfigBits::{TxClearAbt};
        let mut dirty_tx: u32 = self.dirty_tx;
        let mut tx_left: u32 = self.cur_tx - self.dirty_tx;
        while tx_left > 0
        {
            let entry: u32 = dirty_tx % NUM_TX_DESC as u32;
            let txstatus: TxStatusBits;
            txstatus = u32_to_tx_status_bits(self.RTL_RU32(TxStatus0 as u32 + (entry * 32)));
            
            if !((txstatus as u32 & (TxStatOk as u32 | TxUnderrun as u32 | TxAborted as u32)) == 0)
            {
                break;
            }

            if (txstatus as u32 & TxAborted as u32) == 0
            {
                self.RTL_W32(TxConfig, TxClearAbt as u32);
                self.RTL_W16(IntrStatus, TxErr as u16);
                /* lots of stat logging */
                // if txstatus & TxCarrierLost
                // if txstatus & TxOutOfWindow

                // wmb();
                // Might honestly be assembly
                // Seems to be a holdover from PowerPc
                // Doesn't mean much of anything, but might correspond to the x86 assembly instructions:
                // lfence, sfence, mfence
                // xchgl might be a similar instruction as well
            }
            dirty_tx += 1;
            tx_left -= 1;
        }

        if self.dirty_tx != dirty_tx
        {
            self.dirty_tx = dirty_tx;
            // mb() ?
            // netif_wake_queue();
        }
    }

    // Note using an 8K ring... probably
    pub unsafe fn open(&self)
    {
        /* Currently passing ioaddr to everything */
        /* Would be better as a struct... */
        // tp->tx_bufs = dma_alloc_coherent(&tp->pci_dev->dev, TX_BUF_TOT_LEN,
        //				   &tp->tx_bufs_dma, GFP_KERNEL);
        // Unused:
        // let txbufs: heapless::Vec<U1024>; /* Change to TX_BUF_TOL_LEN later */
        // let rxring: heapless::Vec<U1024>; /* Change to RX_BUF_TOT_LEN later */
        // Should check from memory allocation errors, but won't for now
        
        // init_ring();
        self.hw_start();
        // netif_start_queue()
        // start_thread()
    }
}