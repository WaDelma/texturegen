use shader::Context;
use process::{Process, Setting, SettingMut};

#[derive(Clone, Debug)]
pub struct Blend(Type, Type);

impl Blend {
    pub fn new(color_blend: Type, alpha_blend: Type) -> Box<Process + Sized> {
        Box::new(Blend(color_blend, alpha_blend))
    }
}

impl Process for Blend {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "blend" => Blend(&self.0),
            "alpha" => Blend(&self.1),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "blend" => Blend(&mut self.0),
            "alpha" => Blend(&mut self.1),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["blend", "alpha"]
    }
    fn max_in(&self) -> u32 {2}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        if ctx.input_len() == 0 {
            return format!("vec4 {} = vec4(0);\n", ctx.output(0));
        }
        if ctx.input_len() == 1 {
            return format!("vec4 {} = {};\n", ctx.output(0), ctx.first_input());
        }
        let mut result = format!("vec4 {} = vec4(", ctx.output(0));
        result.push_str(&self.0.blend(ctx, "rgb"));
        result.push_str(",\n");
        result.push_str(&self.1.blend(ctx, "a"));
        result.push_str(");\n");
        result
    }
}

custom_derive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq,
        IterVariants(Types), IterVariantNames(TypeNames))]
    pub enum Type {
        Normal,
        Multiply,
        Divide,
        Add,
        Substract,
        Difference,
        Darken,
        Lighten,
        Screen,
        Overlay,
        Hard,
        Soft,
        // Dodge,
        // Burn,
    }
}

impl Type {
    fn blend(&self, ctx: &mut Context, channels: &str) -> String {
        use self::Type::*;
        let a = format!("{}.{}", ctx.input(0).unwrap(), channels);
        let b = format!("{}.{}", ctx.input(1).unwrap(), channels);
        let one = format!("one.{}", channels);
        match *self {
            Normal =>     format!("{}", b),
            Multiply =>   format!("{} * {}", a, b),
            Divide =>     format!("{} / {}", a, b),
            Add =>        format!("{} + {}", a, b),
            Substract =>  format!("{} - {}", a, b),
            Difference => format!("abs({} - {})", a, b),
            Darken =>     format!("min({}, {})", a, b),
            Lighten =>    format!("max({}, {})", a, b),
            Screen =>     format!("{one} - ({one} - {}) * ({one} - {})", a, b, one = one),
            Overlay =>    for_each_channel(channels, |c| {
                              let a = format!("{}.{}", ctx.input(0).unwrap(), c);
                              let b = format!("{}.{}", ctx.input(1).unwrap(), c);
                              let one = format!("one.{}", c);
                              format!("{a} < 0.5?\n\
                                    (2 * {a} * {b}):\n\
                                    ({one} - 2 * ({one} - {a}) * ({one} - {b}))", a = a, b = b, one = one)
                          }),
            Hard =>       for_each_channel(channels, |c| {
                              let a = format!("{}.{}", ctx.input(0).unwrap(), c);
                              let b = format!("{}.{}", ctx.input(1).unwrap(), c);
                              let one = format!("one.{}", c);
                              format!("{b} < 0.5?\n\
                              (2 * {a} * {b}):\n\
                              ({one} - 2 * ({one} - {a}) * ({one} - {b}))", a = a, b = b, one = one)
                          }),
            Soft =>       for_each_channel(channels, |c| {
                              let a = format!("{}.{}", ctx.input(0).unwrap(), c);
                              let b = format!("{}.{}", ctx.input(1).unwrap(), c);
                              format!("{b} < 0.5?\n\
                              (2 * {a} * {b} + {a} * {a} - 2 * {a} * {a} * {b}):\n\
                              (2 * sqrt({a}) * {b} - sqrt({a}) + 2 * {a} - 2 * {a} * {b})", a = a, b = b)
                          }),
            // b => panic!("Blending mode \"{:?}\" has not been implemented.", b),
        }
    }
}

fn for_each_channel<F: FnMut(char) -> String>(channels: &str, mut fun: F) -> String {
    let mut result = match channels.len() {
        1 => String::new(),
        n @ 2...4 => format!("vec{}(\n", n),
        n => panic!("Invalid amount of channels: {}", n),
    };
    let mut first = true;
    for c in channels.chars() {
        if !first {
            result.push_str(",\n");
        }
        result.push_str(&fun(c));
        first = false;
    }
    if channels.len() > 1 {
        result.push(')');
    }
    result
}
