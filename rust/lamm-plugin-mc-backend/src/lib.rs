use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use std::io::Cursor;

#[no_mangle]
pub extern "system" fn Java_com_minneelyyyy_plugin_Lamm_runCode(
    mut env: JNIEnv,
    _class: JClass,
    input: JString) -> jstring {
    let code: String =
        env.get_string(&input).expect("Couldn't get java string!").into();

    let runtime = lamm::Runtime::new(Cursor::new(code), "<eval>");

    let values = runtime.values()
        .map(|res| res.map(|value| value.to_string()))
        .collect::<Result<Vec<_>, _>>();

    let result = match values {
        Ok(values) => values.join("\n"),
        Err(_e) => "an error occurred!".to_string(),
    };

    let output = env.new_string(result)
        .expect("Couldn't create java string!");
    output.into_raw()
}