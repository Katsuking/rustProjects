// product && creator

/// Dialog has a factory method `create_button`.
/// It creates different buttons depending on a factory implementation.

pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

pub trait Dialog {
    // それぞれのインスタンスを Box<dyn Sound> にラップすることで、
    // 異なる型の値を同じトレイトオブジェクトとして扱うことができます。
    fn create_button(&self) -> Box<dyn Button>;
}
