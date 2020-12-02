#![allow(non_camel_case_types)]

use std::{os::raw::c_char, ffi::c_void};

pub const IBV_SYSFS_NAME_MAX: usize	= 64;
pub const IBV_SYSFS_PATH_MAX: usize	= 256;

#[repr(C)]
struct ibv_context_ops{
    _compat_query_device: extern fn(),
    //_compat_query_port: extern fn(context: *const ibv_context, port_num: u8, port_attr: *const _compat_ibv_port_attr) -> i32,

    /*void *(*_compat_alloc_pd)(void);
    void *(*_compat_dealloc_pd)(void);
    void *(*_compat_reg_mr)(void);
    void *(*_compat_rereg_mr)(void);
    void *(*_compat_dereg_mr)(void);
    struct ibv_mw *		(*alloc_mw)(struct ibv_pd *pd, enum ibv_mw_type type);
    int			(*bind_mw)(struct ibv_qp *qp, struct ibv_mw *mw,
                        struct ibv_mw_bind *mw_bind);
    int			(*dealloc_mw)(struct ibv_mw *mw);
    void *(*_compat_create_cq)(void);
    int			(*poll_cq)(struct ibv_cq *cq, int num_entries, struct ibv_wc *wc);
    int			(*req_notify_cq)(struct ibv_cq *cq, int solicited_only);
    void *(*_compat_cq_event)(void);
    void *(*_compat_resize_cq)(void);
    void *(*_compat_destroy_cq)(void);
    void *(*_compat_create_srq)(void);
    void *(*_compat_modify_srq)(void);
    void *(*_compat_query_srq)(void);
    void *(*_compat_destroy_srq)(void);
    int			(*post_srq_recv)(struct ibv_srq *srq,
                            struct ibv_recv_wr *recv_wr,
                            struct ibv_recv_wr **bad_recv_wr);
    void *(*_compat_create_qp)(void);
    void *(*_compat_query_qp)(void);
    void *(*_compat_modify_qp)(void);
    void *(*_compat_destroy_qp)(void);
    int			(*post_send)(struct ibv_qp *qp, struct ibv_send_wr *wr,
                            struct ibv_send_wr **bad_wr);
    int			(*post_recv)(struct ibv_qp *qp, struct ibv_recv_wr *wr,
                            struct ibv_recv_wr **bad_wr);
    void *(*_compat_create_ah)(void);
    void *(*_compat_destroy_ah)(void);
    void *(*_compat_attach_mcast)(void);
    void *(*_compat_detach_mcast)(void);
    void *(*_compat_async_event)(void);*/
}

#[repr(C)]
pub struct ibv_context {
    device: *const ibv_device,
    ops: ibv_context_ops,
    cmd_fd: i32,
    async_fd: i32,
    num_comp_vectors: i32,
    mutex: libc::pthread_mutex_t,
    abi_compat: *const c_void,
}

#[repr(C)]
pub struct _ibv_device_ops {
    dummy1: extern fn(device: *const ibv_device, cmd_fd: i32) -> *const ibv_context,
    dummy2: extern fn(context: *const ibv_context),
}

#[repr(C)]
pub enum ibv_node_type {
    IBV_NODE_UNKNOWN	= -1,
    IBV_NODE_CA 		= 1,
    IBV_NODE_SWITCH,
    IBV_NODE_ROUTER,
    IBV_NODE_RNIC,
    IBV_NODE_USNIC,
    IBV_NODE_USNIC_UDP,
    IBV_NODE_UNSPECIFIED,
}

pub use ibv_node_type::*;

#[repr(C)]
pub enum ibv_transport_type {
    IBV_TRANSPORT_UNKNOWN	= -1,
    IBV_TRANSPORT_IB	= 0,
    IBV_TRANSPORT_IWARP,
    IBV_TRANSPORT_USNIC,
    IBV_TRANSPORT_USNIC_UDP,
    IBV_TRANSPORT_UNSPECIFIED,
}

pub use ibv_transport_type::*;

#[repr(C)]
pub struct ibv_device {
    _ops: _ibv_device_ops,
    node_type: ibv_node_type,
    transport_type: ibv_transport_type,
    /// Name of underlying kernel IB device, eg "mthca0"
    name: [u8; IBV_SYSFS_NAME_MAX],
    /// Name of uverbs device, eg "uverbs0" */
    dev_name: [u8; IBV_SYSFS_NAME_MAX],
    /// Path to infiniband_verbs class device in sysfs
    dev_path: [u8; IBV_SYSFS_PATH_MAX],
    /// Path to infiniband class device in sysfs
    ibdev_path: [u8; IBV_SYSFS_PATH_MAX],
}

extern "C" {
    pub fn ibv_get_device_list(num_devices: *mut i32) -> *const *const ibv_device;
    pub fn ibv_free_device_list(list: *const *const ibv_device);
    pub fn ibv_get_device_name(device: *const ibv_device) -> *const c_char;
    pub fn ibv_get_device_guid(device: *const ibv_device) -> u64;
    pub fn ibv_open_device(device: *const ibv_device) -> *const ibv_context;
    pub fn ibv_close_device(context: *const ibv_context) -> i32;
}
