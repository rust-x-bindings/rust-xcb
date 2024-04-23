(function() {var type_impls = {
"xcb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#659-815\">source</a><a href=\"#impl-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#660-700\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.new\" class=\"fn\">new</a>(\n    event_base: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>,\n    detail: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>,\n    time: <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a>,\n    root: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    event: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    child: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    root_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a>,\n    root_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a>,\n    event_x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a>,\n    event_y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a>,\n    state: <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a>,\n    same_screen: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a>,\n    device_id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>\n) -&gt; <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h4></section><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#706-712\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.detail\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#714-720\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.detail\" class=\"fn\">detail</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#722-728\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.time\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#730-736\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.time\" class=\"fn\">time</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a></h4></section><section id=\"method.root\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#738-744\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.root\" class=\"fn\">root</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.event\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#746-752\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.event\" class=\"fn\">event</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.child\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#754-760\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.child\" class=\"fn\">child</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.root_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#762-768\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.root_x\" class=\"fn\">root_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a></h4></section><section id=\"method.root_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#770-776\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.root_y\" class=\"fn\">root_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a></h4></section><section id=\"method.event_x\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#778-784\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.event_x\" class=\"fn\">event_x</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a></h4></section><section id=\"method.event_y\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#786-792\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.event_y\" class=\"fn\">event_y</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.i16.html\">i16</a></h4></section><section id=\"method.state\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#794-801\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.state\" class=\"fn\">state</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.KeyButMask.html\" title=\"struct xcb::x::KeyButMask\">KeyButMask</a></h4></section><section id=\"method.same_screen\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#803-806\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.same_screen\" class=\"fn\">same_screen</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section><section id=\"method.device_id\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#808-814\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceKeyPressEvent.html#tymethod.device_id\" class=\"fn\">device_id</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a></h4></section></div></details>",0,"xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_event_t%3E-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#644-652\">source</a><a href=\"#impl-Raw%3Cxcb_generic_event_t%3E-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#645-647\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#649-651\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_event_t>","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#817-836\">source</a><a href=\"#impl-Debug-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#818-835\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<section id=\"impl-Send-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#876\">source</a><a href=\"#impl-Send-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section>","Send","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseEvent-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#654-657\">source</a><a href=\"#impl-BaseEvent-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseEvent.html\" title=\"trait xcb::BaseEvent\">BaseEvent</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#655\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = _</h4></section></summary><div class='docblock'>The extension associated to this event, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#656\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u32.html\">u32</a> = 1u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_from_send_event\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#87-91\">source</a><a href=\"#method.is_from_send_event\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.BaseEvent.html#method.is_from_send_event\" class=\"fn\">is_from_send_event</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Check whether this event was emitted by the SendEvent request\nSee <code>[crate::x::SendEvent]</code>.</div></details></div></details>","BaseEvent","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<section id=\"impl-Sync-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#877\">source</a><a href=\"#impl-Sync-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section>","Sync","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-DeviceKeyPressEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#868-874\">source</a><a href=\"#impl-Drop-for-DeviceKeyPressEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceKeyPressEvent.html\" title=\"struct xcb::xinput::DeviceKeyPressEvent\">DeviceKeyPressEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-d9f33c9837f06485/out/xinput.rs.html#869-873\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.77.2/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xinput::DeviceKeyReleaseEvent","xcb::xinput::DeviceButtonPressEvent","xcb::xinput::DeviceButtonReleaseEvent","xcb::xinput::DeviceMotionNotifyEvent","xcb::xinput::ProximityInEvent","xcb::xinput::ProximityOutEvent"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()