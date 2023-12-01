# virt2slint

这个crate用于将virtio设备的规范转换为slint的规范。

virtio规范描述了输入事件的格式形式如下:
    
```rust
pub struct InputEvent {
    /// Event type.
    pub event_type: u16,
    /// Event code.
    pub code: u16,
    /// Event value.
    pub value: u32,
}
```
我们借助`virtio-input-decoder` crate 将内核传递的`u64`表示的`InputEvent`解析为对应的键盘或者鼠标事件。

核心的数据结构是`Converter`:
```rust
pub struct Converter {
    x_res:isize,
    y_res:isize,
    virtual_range:isize,
}
```
我们需要知道屏幕的分辨率，以及鼠标的移动范围，才能将鼠标的移动转换为屏幕的移动。对于`virtio-tablet-device` 设备，鼠标的移动会传递绝对坐标；
对于`virtio-mouse-device`设备，鼠标的移动会传递相对坐标。鉴于相对坐标的管理复杂度，我们目前只支持绝对坐标，也就是`virtio-tablet-device`设备。

对于qemu模拟器，鼠标输入事件的坐标单位并不是与屏幕的分辨率一致的，而是一个虚拟的坐标范围，我们需要将其转换为屏幕的坐标范围。通常，这个
虚拟的坐标范围是`[0, 32767]`，对于横坐标/纵坐标都是这样。因此简单的转换格式是：

```rust
screencoordx = (x_res * virtio_x) / virtual_range
screencoordy = (y_res * virtio_y) / virtual_range
```

Usage:

```rust
use virt2slint::Converter;
let converter = Converter::new(32767,1200,800);
let mut x = 0;
let mut y = 0;
let event = converter.convert(0x0,&mut x,&mut y).unwrap();
```

