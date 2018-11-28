use crate::netdevice;
 
struct e1000_adapter;

const E1000_MAX_INTR: u8 = 10;

/*
 * Count for polling __E1000_RESET condition every 10-20msec.
 */
const E1000_CHECK_RESET_COUNT: u8 = 50;

/* TX/RX descriptor defines */
// What even is TX/RX descriptors?
const E1000_DEFAULT_TXD: u8 = 256;
const E1000_MAX_TXD: u8 = 256;
const E1000_MIN_TXD: u8 = 48;
const E1000_MAX_82544_TXD: u16 = 4096;

const E1000_DEFAULT_RXD = 256;
const E1000_MAX_RXD: u8 = 256;
const E1000_MIN_RXD: u8 = 48;
const E1000_MAX_82544_RXD: u16 = 4096;

const E1000_MIN_ITR_USECS: u8 =	10; 
/* 100000 irq/sec */
const E1000_MAX_ITR_USECS: u8 = 10000; 
/* 100    irq/sec */

/* this is the size past which hardware will drop packets when setting LPE=0 */
const MAXIMUM_ETHERNET_VLAN_SIZE: u8 = 1522;


/* Supported Rx Buffer Sizes */
const E1000_RXBUFFER_128: u8 = 128;    
/* Used for packet split */
const E1000_RXBUFFER_256: u8 = 256;    
/* Used for packet split */
const E1000_RXBUFFER_512: u8 = 512;
const E1000_RXBUFFER_1024: u8 = 1024;
const E1000_RXBUFFER_2048: u8 = 2048;
const E1000_RXBUFFER_4096: u8 = 4096;
const E1000_RXBUFFER_8192: u8 = 8192;
const E1000_RXBUFFER_16384: u8 = 16384;

/* SmartSpeed delimiters */
const E1000_SMARTSPEED_DOWNSHIFT: u8 = 3;
const E1000_SMARTSPEED_MAX: u8 = 15;

/* Packet Buffer allocations */
const E1000_PBA_BYTES_SHIFT: u8 = 0xA;
const E1000_TX_HEAD_ADDR_SHIFT: u8 = 7;
const E1000_PBA_TX_MASK: u8 = 0xFFFF0000;

/* Flow Control Watermarks */
const E1000_FC_HIGH_DIFF: u8 = 0x1638; 
/* High: 5688 bytes below Rx FIFO size */
const E1000_FC_LOW_DIFF: u8 = 0x1640; 
/* Low:  5696 bytes below Rx FIFO size */

const E1000_FC_PAUSE_TIME: u8 = 0xFFFF;
/* pause for the max or until send xon */

/* How many Tx Descriptors do we need to call netif_wake_queue ? */
const E1000_TX_QUEUE_WAKE: u8 = 16;
/* How many Rx Buffers do we bundle into one write to the hardware ? */
const E1000_RX_BUFFER_WRITE: u8 = 16;
/* Must be power of 2 */

const AUTO_ALL_MODES: u8 = 0;
const E1000_EEPROM_82544_APM: u8 = 0x0004;
const E1000_EEPROM_APME: u8 = 0x0400;

// #ifndef E1000_MASTER_SLAVE
/* Switch to override PHY master/slave setting */
// #define E1000_MASTER_SLAVE	e1000_ms_hw_default
// #endif

const E1000_MNG_VLAN_NONE: i8 =	(-1);

/* wrapper around a pointer to a socket buffer,
 * so a DMA handle can be stored along with the buffer
 */
struct e1000_tx_buffer {
	// struct sk_buff *skb;
	// dma : dma_addr_t,
	time_stamp : u64, // unsigned long ;
	length: u16,
	next_to_watch: u16 ,
	mapped_as_page: bool,
	segs : u8,
	bytecount : u32;
}

// Has to be a better way than a union here...
struct e1000_rx_buffer {
    // union {
	//	struct page *page; /* jumbo: alloc_page */
	// 	u8 *data; /* else, netdev_alloc_frag */
	// } rxbuf;
	// dma_addr_t dma;
}

struct e1000_tx_ring {
    /* pointer to the descriptor ring memory */
	// void *desc;
	/* physical address of the descriptor ring */
	// dma_addr_t dma;
	/* length of descriptor ring in bytes */
	size: u32,
	/* number of descriptors in the ring */
	count: u32,
	/* next descriptor to associate a buffer with */
	next_to_use: u32,
	/* next descriptor to check for DD status bit */
	next_to_clean: u32,
	/* array of buffer information structs */
    // Need some smart pointers yo
	// struct e1000_tx_buffer *buffer_info;
	tdh: u16, // What are thoseeeeeee
	tdt: u16,
	last_tx_tso: bool,
}

struct e1000_rx_ring {
	/* pointer to the descriptor ring memory */
	// void *desc;
	/* physical address of the descriptor ring */
	// dma_addr_t dma;
	/* length of descriptor ring in bytes */
	size: u32,
	/* number of descriptors in the ring */
	count: u32,
	/* next descriptor to associate a buffer with */
	next_to_use: u32,
	/* next descriptor to check for DD status bit */
	next_to_clean: u32,
	/* array of buffer information structs */
	// struct e1000_rx_buffer *buffer_info;
	// struct sk_buff *rx_skb_top;

	/* cpu for rx queue */
	cpu: u32,

	rdh: u16,
	rdt: u16,
};

/* board specific private data structure */

struct e1000_adapter {
	unsigned long active_vlans[BITS_TO_LONGS(VLAN_N_VID)];
	u16 mng_vlan_id;
	u32 bd_number;
	u32 rx_buffer_len;
	u32 wol;
	u32 smartspeed;
	u32 en_mng_pt;
	u16 link_speed;
	u16 link_duplex;
	spinlock_t stats_lock;
	unsigned int total_tx_bytes;
	unsigned int total_tx_packets;
	unsigned int total_rx_bytes;
	unsigned int total_rx_packets;
	/* Interrupt Throttle Rate */
	u32 itr;
	u32 itr_setting;
	u16 tx_itr;
	u16 rx_itr;

	u8 fc_autoneg;

	/* TX */
	struct e1000_tx_ring *tx_ring;      /* One per active queue */
	unsigned int restart_queue;
	u32 txd_cmd;
	u32 tx_int_delay;
	u32 tx_abs_int_delay;
	u32 gotcl;
	u64 gotcl_old;
	u64 tpt_old;
	u64 colc_old;
	u32 tx_timeout_count;
	u32 tx_fifo_head;
	u32 tx_head_addr;
	u32 tx_fifo_size;
	u8  tx_timeout_factor;
	atomic_t tx_fifo_stall;
	bool pcix_82544;
	bool detect_tx_hung;
	bool dump_buffers;

	/* RX */
	bool (*clean_rx)(struct e1000_adapter *adapter,
			 struct e1000_rx_ring *rx_ring,
			 int *work_done, int work_to_do);
	void (*alloc_rx_buf)(struct e1000_adapter *adapter,
			     struct e1000_rx_ring *rx_ring,
			     int cleaned_count);
	struct e1000_rx_ring *rx_ring;      /* One per active queue */
	struct napi_struct napi;

	int num_tx_queues;
	int num_rx_queues;

	u64 hw_csum_err;
	u64 hw_csum_good;
	u32 alloc_rx_buff_failed;
	u32 rx_int_delay;
	u32 rx_abs_int_delay;
	bool rx_csum;
	u32 gorcl;
	u64 gorcl_old;

	/* OS defined structs */
	struct net_device *netdev;
	struct pci_dev *pdev;

	/* structs defined in e1000_hw.h */
	struct e1000_hw hw;
	struct e1000_hw_stats stats;
	struct e1000_phy_info phy_info;
	struct e1000_phy_stats phy_stats;

	u32 test_icr;
	struct e1000_tx_ring test_tx_ring;
	struct e1000_rx_ring test_rx_ring;

	int msg_enable;

	/* to not mess up cache alignment, always add to the bottom */
	bool tso_force;
	bool smart_power_down;	/* phy smart power down */
	bool quad_port_a;
	unsigned long flags;
	u32 eeprom_wol;

	/* for ioport free */
	int bars;
	int need_ioport;

	bool discarding;

	struct work_struct reset_task;
	struct delayed_work watchdog_task;
	struct delayed_work fifo_stall_task;
	struct delayed_work phy_info_task;
};

// int e1000_open(struct net_device *netdev)
fn e1000_open(netdev: net_device) -> i32
{

}

// int e1000_close(struct net_device *netdev);
fn e1000_close(netdev: net_device) -> i32
{

}

// int e1000_up(struct e1000_adapter *adapter);
fn e1000_up(adapter: e1000_adapter) -> i32
{

}

// void e1000_down(struct e1000_adapter *adapter);
fn e1000_down(adapter: e1000_adapter)
{

}

// void e1000_reinit_locked(struct e1000_adapter *adapter);
fn e1000_adapter_reinit_locked(adapter: e1000_adapter)
{

}
// void e1000_reset(struct e1000_adapter *adapter);
fn e1000_reset(adapter: e1000_adapter)
{

}

// int e1000_set_spd_dplx(struct e1000_adapter *adapter, u32 spd, u8 dplx);
fn e1000_set_spd_dplx(adapter: e1000_adapter, spd: u32, dplx: u8) -> i32
{

}

// int e1000_setup_all_rx_resources(struct e1000_adapter *adapter);
fn e1000_setup_all_rx_resources(adapter: e1000_adapter) -> i32
{

}

// int e1000_setup_all_tx_resources(struct e1000_adapter *adapter);
fn e1000_setup_all_tx_resources(adapter: e1000_adapter) -> i32
{

}


// void e1000_free_all_rx_resources(struct e1000_adapter *adapter);
fn e1000_free_all_rx_resources(adapter: e1000_adapter)
{

}

// void e1000_free_all_tx_resources(struct e1000_adapter *adapter);
fn e1000_free_all_tx_resources(adapter: e1000_adapter)
{

}

// void e1000_update_stats(struct e1000_adapter *adapter);
fn e1000_update_stats(adapter: e1000_adapter)
{

}

// bool e1000_has_link(struct e1000_adapter *adapter);
fn e1000_has_link(adapter: e1000_adapter) -> bool
{

}
// void e1000_power_up_phy(struct e1000_adapter *);
fn e1000_power_up_phy(adapter: e1000_adapter)
{

}
// void e1000_set_ethtool_ops(struct net_device *netdev);
fn e1000_set_ethtool_ops(netdev: net_device)
{

}

// void e1000_check_options(struct e1000_adapter *adapter);
fn e1000_check_options(adapter: e1000_adapter)
{

}
// char *e1000_get_hw_dev_name(struct e1000_hw *hw);

fn e1000_get_hw_dev_name(hw: e1000_hw) -> String
{

}