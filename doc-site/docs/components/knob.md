# RDSKnob

A rotary knob supporting bounded and infinite rotation modes.

## Properties

- **value**: `float`
- **min-value**: `float`
- **max-value**: `float`
- **size**: `length`
- **rotation-angle**: `angle`
- **indicator-position**: `length`
- **sensitivity**: `float`
- **scroll-sensitivity**: `float`
- **renderer**: `string` ("hi-fi", "lo-fi")
- **indicator**: `string` ("circle", "line", "caret")
- **indicator-color**: `color`
- **label**: `string` - Text label for the knob
- **label-pos**: `string` (Default: `"bottom-center"`) - Position of the label ("top-left", "top-center", "top-right", "center-left", "center-center", "center-right", "bottom-left", "bottom-center", "bottom-right")
- **label-width**: `length` - Custom width for label area
- **label-height**: `length` - Custom height for label area
- **label-txt-color**: `color` - Color of the label text
- **label-bg-color**: `color` - Background color of the label area
- **label-font-size**: `length` - Font size for the label
- **label-font-weight**: `int` - Font weight for the label

## Callbacks

- **value-changed(float)**: Triggered when the value is modified.

## Example

```slint
import { RDSKnob } from "rds-slint-ui-kit/components/index.slint";

RDSKnob {
    value: 50;
    min-value: 0;
    max-value: 100;
    size: 64px;
    value-changed(v) => { /* handle value */ }
}
```
