# PrusaSlicer M221 per Object

This is a very simple g-code post-processor which enables you to change the flow rate per object.

It adds `M221 S[flow rate]` after each object start. 

`[flow rate]` is parsed as a float from the start of the object name until a whitespace or "_" as separator.

If `[flow rate]` shall include a decimal point you need to replace it by `p` or `P`.

If no valid `[flow rate]` is found the object will be printed with flow rate 100%.

## Slicer Configuration

### Print Settings - Output Options

#### Label Objects

![Label Objects](./doc/label_objects.png)

#### Post Processing Script

![Post Processing Script](./doc/post_processing_script.png)
