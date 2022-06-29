use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_star_StarVpnService_runLeaf(
    env: JNIEnv,
    _: JClass,
    config_path: JString,
    protect_path: JString,
) {
    let config_path = env
        .get_string(config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let protect_path = env
        .get_string(protect_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let opts = leaf::StartOptions {
        config: leaf::Config::File(config_path),
        socket_protect_path: Some(protect_path),
        runtime_opt: leaf::RuntimeOption::MultiThreadAuto(6),
    };
    leaf::start(10086, opts).unwrap();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_star_StarVpnService_stopLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::shutdown(10086);
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_star_StarVpnService_reloadLeaf(
    _: JNIEnv,
    _: JClass,
) {
    leaf::reload(10086).unwrap();
}
