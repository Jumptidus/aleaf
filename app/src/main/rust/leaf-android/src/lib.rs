use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_runLeaf(
    mut env: JNIEnv,
    _: JClass,
    config_path: JString,
    _protect_path: JString,
) {
    let config_path = env
        .get_string(config_path.as_ref())
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    // let protect_path = env
    //     .get_string(_protect_path)
    //     .unwrap()
    //     .to_str()
    //     .unwrap()
    //     .to_owned();
    let opts = leaf::StartOptions {
        config: leaf::Config::File(config_path),
        // socket_protect_path: Some(protect_path),
        auto_reload: true,
        runtime_opt: leaf::RuntimeOption::MultiThreadAuto(2),
    };
    leaf::start(10086, opts).unwrap();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_stopLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::shutdown(10086);
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_noob_NoobVpnService_reloadLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::reload(10086).unwrap();
}
