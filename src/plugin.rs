use log::{debug, error};
use samp::native;
use samp::prelude::*;
use samp::SampPlugin;
use serde_yaml;
use std::sync::{Arc, Mutex};

use crate::pool::GarbageCollectedPool;

pub struct Plugin {
    pub yaml_nodes: Arc<Mutex<GarbageCollectedPool<serde_yaml::Value>>>,
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Clone)]
enum YamlNode {
    Number = 0,
    Boolean,
    String,
    Object,
    Array,
    Null,
}
}

impl SampPlugin for Plugin {}

impl Plugin {
    #[native(name = "YAML_Parse")]
    pub fn yaml_parse(&mut self, _: &Amx, input: AmxString, mut node: Ref<i32>) -> AmxResult<i32> {
        let v: serde_yaml::Value = match serde_yaml::from_str(&input.to_string()) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(1);
            }
        };

        let mut nodes = self.yaml_nodes.lock().unwrap();
        *node = nodes.alloc(v);

        Ok(0)
    }

    #[native(name = "YAML_Stringify")]
    pub fn yaml_stringify(
        &mut self,
        _: &Amx,
        node: i32,
        output: UnsizedBuffer,
        length: usize,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: &serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };

        let s = match serde_yaml::to_string(&v) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(1);
            }
        };

        let mut dest = output.into_sized_buffer(length);
        let _ = samp::cell::string::put_in_buffer(&mut dest, &s);

        Ok(0)
    }

    #[native(name = "YAML_NodeType")]
    pub fn yaml_node_type(&mut self, _: &Amx, node: i32) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();
        let v: &serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => &serde_yaml::Value::Null,
        };

        debug!("{:?}", v);

        let t: i32 = match v {
            serde_yaml::Value::Null => YamlNode::Null as i32,
            serde_yaml::Value::Bool(_) => YamlNode::Boolean as i32,
            serde_yaml::Value::Number(_) => YamlNode::Number as i32,
            serde_yaml::Value::String(_) => YamlNode::String as i32,
            serde_yaml::Value::Sequence(_) => YamlNode::Array as i32,
            serde_yaml::Value::Mapping(_) => YamlNode::Object as i32,
        };

        Ok(t)
    }

    #[native(raw, name = "YAML_Object")]
    pub fn yaml_object(&mut self, _: &Amx, mut params: samp::args::Args) -> AmxResult<i32> {
        let arg_count = params.count();
        let pairs = if arg_count == 0 || arg_count % 2 == 0 {
            arg_count / 2
        } else {
            error!("invalid variadic argument pattern passed to YAML_Object");
            return Ok(1);
        };

        let mut v = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
        for _ in 0..pairs {
            let key = match params.next::<AmxString>() {
                None => {
                    error!("invalid type expected String");
                    return Ok(2);
                }
                Some(parameter) => parameter,
            };

            let node = match params.next::<Ref<i32>>() {
                None => {
                    error!("invalid type expected int");
                    return Ok(2);
                }
                Some(parameter) => parameter,
            };

            let mut nodes = self.yaml_nodes.lock().unwrap();

            let node = match nodes.take(*node) {
                Some(v) => v,
                None => {
                    error!("invalid YAML node ID passed to YAML_Object");
                    return Ok(2);
                }
            };

            v[key.to_string()] = node.clone();
        }

        let mut nodes = self.yaml_nodes.lock().unwrap();
        Ok(nodes.alloc(v))
    }

    #[native(name = "YAML_Int")]
    pub fn yaml_int(&mut self, _: &Amx, value: i32) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();
        Ok(nodes.alloc(serde_yaml::to_value(value).unwrap()))
    }

    #[native(name = "YAML_Bool")]
    pub fn yaml_bool(&mut self, _: &Amx, value: bool) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();
        Ok(nodes.alloc(serde_yaml::to_value(value).unwrap()))
    }

    #[native(name = "YAML_Float")]
    pub fn yaml_float(&mut self, _: &Amx, value: f32) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();
        Ok(nodes.alloc(serde_yaml::to_value(value).unwrap()))
    }

    #[native(name = "YAML_String")]
    pub fn yaml_string(&mut self, _: &Amx, value: AmxString) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();
        Ok(nodes.alloc(serde_yaml::to_value(value.to_string()).unwrap()))
    }

    #[native(raw, name = "YAML_Array")]
    pub fn yaml_array(&mut self, _: &Amx, mut params: samp::args::Args) -> AmxResult<i32> {
        let args = params.count();

        let mut arr = Vec::<serde_yaml::Value>::new();
        for _ in 0..args {
            let node = match params.next::<Ref<i32>>() {
                None => {
                    error!("invalid type expected int");
                    return Ok(1);
                }
                Some(parameter) => parameter,
            };

            let mut nodes = self.yaml_nodes.lock().unwrap();
            let node = match nodes.take(*node) {
                Some(v) => v,
                None => {
                    error!("invalid YAML node ID passed to YAML_Array");
                    return Ok(1);
                }
            };
            arr.push(node.clone());
        }

        let mut nodes = self.yaml_nodes.lock().unwrap();

        Ok(nodes.alloc(serde_yaml::Value::Sequence(arr)))
    }

    #[native(name = "YAML_Append")]
    pub fn yaml_append(&mut self, _: &Amx, a: i32, b: i32) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let a: serde_yaml::Value = match nodes.take(a) {
            Some(v) => v,
            None => return Ok(-1),
        };
        let b: serde_yaml::Value = match nodes.take(b) {
            Some(v) => v,
            None => return Ok(-1),
        };

        match (a.as_mapping(), b.as_mapping()) {
            (Some(oa), Some(ob)) => {
                let mut new = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
                for (k, v) in oa.iter() {
                    new.as_mapping_mut().unwrap().insert(k.clone(), v.clone());
                }
                for (k, v) in ob.iter() {
                    new.as_mapping_mut().unwrap().insert(k.clone(), v.clone());
                }
                return Ok(nodes.alloc(new));
            }
            _ => debug!("append: a and b are not both objects"),
        };

        match (a.as_sequence(), b.as_sequence()) {
            (Some(oa), Some(ob)) => {
                let mut new = serde_yaml::Value::Sequence(Vec::new());
                for v in oa.iter() {
                    new.as_sequence_mut().unwrap().push(v.clone());
                }
                for v in ob.iter() {
                    new.as_sequence_mut().unwrap().push(v.clone());
                }
                return Ok(nodes.alloc(new));
            }
            _ => debug!("append: a and b are not both arrays"),
        };

        debug!("failed to append: a and b are not both objects or arrays");

        Ok(2)
    }

    #[native(name = "YAML_SetObject")]
    pub fn yaml_set_object(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: i32,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let src: serde_yaml::Value = match nodes.take(value) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let dst: &mut serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };
        if !src.is_mapping() || !dst.is_mapping() {
            return Ok(1);
        }

        dst[key.to_string()] = src;
        Ok(0)
    }

    #[native(name = "YAML_SetInt")]
    pub fn yaml_set_int(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: i32,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: &mut serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };
        if !v.is_mapping() {
            return Ok(1);
        }

        v[key.to_string()] = serde_yaml::to_value(value).unwrap();
        Ok(0)
    }

    #[native(name = "YAML_SetFloat")]
    pub fn yaml_set_float(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: f32,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: &mut serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };
        if !v.is_mapping() {
            return Ok(1);
        }

        v[key.to_string()] = serde_yaml::to_value(value).unwrap();
        Ok(0)
    }

    #[native(name = "YAML_SetBool")]
    pub fn yaml_set_bool(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: bool,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: &mut serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };
        if !v.is_mapping() {
            return Ok(1);
        }

        v[key.to_string()] = serde_yaml::to_value(value).unwrap();
        Ok(0)
    }

    #[native(name = "YAML_SetString")]
    pub fn yaml_set_string(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: AmxString,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: &mut serde_yaml::Value = match nodes.get(node) {
            Some(v) => v,
            None => return Ok(1),
        };
        if !v.is_mapping() {
            return Ok(1);
        }

        v[key.to_string()] = serde_yaml::to_value(value.to_string()).unwrap();
        Ok(0)
    }

    #[native(name = "YAML_GetObject")]
    pub fn yaml_get_object(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        mut value: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(2),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(3),
        };
        let v = nodes.alloc(v);
        *value = v;

        Ok(0)
    }

    #[native(name = "YAML_GetInt")]
    pub fn yaml_get_int(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        mut value: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        let v = match v.as_i64() {
            Some(v) => v as i32,
            None => return Ok(3),
        };
        *value = v;

        Ok(0)
    }

    #[native(name = "YAML_GetFloat")]
    pub fn yaml_get_float(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        mut value: Ref<f32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        let v = match v.as_f64() {
            Some(v) => v as f32,
            None => return Ok(3),
        };

        *value = v;

        Ok(0)
    }

    #[native(name = "YAML_GetBool")]
    pub fn yaml_get_bool(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        mut value: Ref<bool>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        let v = match v.as_bool() {
            Some(v) => v,
            None => return Ok(3),
        };
        *value = v;
        Ok(0)
    }

    #[native(name = "YAML_GetString")]
    pub fn yaml_get_string(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        value: UnsizedBuffer,
        length: usize,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        let v = match v.as_str() {
            Some(v) => v,
            None => return Ok(3),
        };

        let mut dest = value.into_sized_buffer(length);
        let _ = samp::cell::string::put_in_buffer(&mut dest, &v);

        Ok(0)
    }

    #[native(name = "YAML_GetArray")]
    pub fn yaml_get_array(
        &mut self,
        _: &Amx,
        node: i32,
        key: AmxString,
        mut value: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_mapping() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(&serde_yaml::Value::String(key.to_string())) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        match v.as_sequence() {
            Some(_) => (),
            None => return Ok(3),
        };
        let v = nodes.alloc(v);
        *value = v;
        Ok(0)
    }

    #[native(name = "YAML_ArrayLength")]
    pub fn yaml_array_length(
        &mut self,
        _: &Amx,
        node: i32,
        mut length: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_sequence() {
            Some(v) => v,
            None => return Ok(1),
        };
        *length = v.len() as i32;
        Ok(0)
    }

    #[native(name = "YAML_ArrayObject")]
    pub fn yaml_array_object(
        &mut self,
        _: &Amx,
        node: i32,
        index: i32,
        mut output: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.get(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_sequence() {
            Some(v) => v,
            None => return Ok(1),
        };
        let v = match v.get(index as usize) {
            Some(v) => v.clone(),
            None => return Ok(2),
        };
        let v = nodes.alloc(v);
        *output = v;
        Ok(0)
    }

    #[native(name = "YAML_GetNodeInt")]
    pub fn yaml_get_node_int(
        &mut self,
        _: &Amx,
        node: i32,
        mut output: Ref<i32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.take(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_i64() {
            Some(v) => v as i32,
            None => return Ok(1),
        };
        *output = v;
        Ok(0)
    }

    #[native(name = "YAML_GetNodeFloat")]
    pub fn yaml_get_node_float(
        &mut self,
        _: &Amx,
        node: i32,
        mut output: Ref<f32>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.take(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_f64() {
            Some(v) => v as f32,
            None => return Ok(1),
        };
        *output = v;
        Ok(0)
    }

    #[native(name = "YAML_GetNodeBool")]
    pub fn yaml_get_node_bool(
        &mut self,
        _: &Amx,
        node: i32,
        mut output: Ref<bool>,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.take(node) {
            Some(v) => v.clone(),
            None => return Ok(1),
        };
        let v = match v.as_bool() {
            Some(v) => v,
            None => return Ok(1),
        };
        *output = v;
        Ok(0)
    }

    #[native(name = "YAML_GetNodeString")]
    pub fn yaml_get_node_string(
        &mut self,
        _: &Amx,
        node: i32,
        output: UnsizedBuffer,
        length: usize,
    ) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        let v: serde_yaml::Value = match nodes.take(node) {
            Some(v) => v.clone(),
            None => {
                debug!("value under {} doesn't exist", node);
                return Ok(1);
            }
        };
        let v = match v.as_str() {
            Some(v) => v,
            None => {
                debug!("value is not a string {:?}", v);
                return Ok(1);
            }
        };
        let mut dest = output.into_sized_buffer(length);
        let _ = samp::cell::string::put_in_buffer(&mut dest, &v);

        Ok(0)
    }

    #[native(name = "YAML_ToggleGC")]
    pub fn yaml_toggle_gc(&mut self, _: &Amx, node: i32, set: bool) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        match nodes.set_gc(node, set) {
            Some(_) => Ok(0),
            None => Ok(1),
        }
    }

    #[native(name = "YAML_Cleanup")]
    pub fn yaml_cleanup(&mut self, _: &Amx, node: i32, auto: bool) -> AmxResult<i32> {
        let mut nodes = self.yaml_nodes.lock().unwrap();

        match if auto {
            nodes.collect(node)
        } else {
            nodes.collect_force(node)
        } {
            Some(_) => Ok(0),
            None => Ok(1),
        }
    }
}
