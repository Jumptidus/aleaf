use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

/// No error.
pub const ERR_OK: i32 = 0;
/// Config path error.
pub const ERR_CONFIG_PATH: i32 = 1;
/// Config parsing error.
pub const ERR_CONFIG: i32 = 2;
/// IO error.
pub const ERR_IO: i32 = 3;
/// Config file watcher error.
pub const ERR_WATCHER: i32 = 4;
/// Async channel send error.
pub const ERR_ASYNC_CHANNEL_SEND: i32 = 5;
/// Sync channel receive error.
pub const ERR_SYNC_CHANNEL_RECV: i32 = 6;
/// Runtime manager error.
pub const ERR_RUNTIME_MANAGER: i32 = 7;
/// No associated config file.
pub const ERR_NO_CONFIG_FILE: i32 = 8;

fn to_errno(e: leaf::Error) -> i32 {
    match e {
        leaf::Error::Config(..) => ERR_CONFIG,
        leaf::Error::NoConfigFile => ERR_NO_CONFIG_FILE,
        leaf::Error::Io(..) => ERR_IO,
        #[cfg(feature = "auto-reload")]
        leaf::Error::Watcher(..) => ERR_WATCHER,
        leaf::Error::AsyncChannelSend(..) => ERR_ASYNC_CHANNEL_SEND,
        leaf::Error::SyncChannelRecv(..) => ERR_SYNC_CHANNEL_RECV,
        leaf::Error::RuntimeManager => ERR_RUNTIME_MANAGER,
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_runLeaf(
    mut env: JNIEnv,
    _: JClass,
    rt_id: u16,
    config_path: JString,
) -> i32 {
    let config_path = env.get_string(config_path.as_ref())
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let opts = leaf::StartOptions {
        config: leaf::Config::File(config_path.to_string()),
        #[cfg(feature = "auto-reload")]
        auto_reload: true,
        runtime_opt: leaf::RuntimeOption::SingleThread,
    };
    if let Err(e) = leaf::start(rt_id, opts) {
        return to_errno(e);
    }
    ERR_OK
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_stopLeaf(
    _: JNIEnv,
    _: JClass,
    rt_id: u16
) -> bool {
    leaf::shutdown(rt_id)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_reloadLeaf(
    _: JNIEnv,
    _: JClass,
    rt_id: u16
) -> i32{
    if let Err(e) = leaf::reload(rt_id){
        return to_errno(e);
    }
    ERR_OK
}
