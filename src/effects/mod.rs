use std::any::Any;
use crate::types::Type;

pub struct Effect {
    pub name: String,
    pub operations: Vec<EffectOperation>,
}

pub struct EffectOperation {
    pub name: String,
    pub input_type: Type,
    pub output_type: Type,
}

pub struct EffectHandler {
    pub effect: Effect,
    pub implementations: Vec<EffectImplementation>,
}

pub struct EffectImplementation {
    pub operation: String,
    pub implementation: Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any>>,
}