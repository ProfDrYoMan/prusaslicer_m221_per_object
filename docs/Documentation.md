# Documentation

The g-code post-processor adds `M221 S[flow rate]` after each object start.
`[flow rate]` is parsed from the start of the object name.

You have two options to set the new flow rate per object.

## Explicit Flow Rates (in %)

* Start the object name with your desired flow rate without a decimal point.
* If you need a decimal point you **need** to replace it by `p` or `P`.
* You are free to use any object name after the new flow rate.
* If parsing fails a flow rate of 100 % is used.

Figure: Explicit Flow Rates From 102 % Down to 98 % in Steps of 0.5 %

![Explicit flow rates](explicit_flow_rates.png)

## Absolut Extrusion Multipliers (~ 1.0)

* Start the object name with your desired extrusion multiplier.
* You **need** to use a decimal point and you **need** to replace it by `m` or `M`.
* You are free to use any object name after the new extrusion multiplier.
* If parsing fails a flow rate of 100 % is used.
* If parsing succeds the new flow rate is calculated out of the desired extrusion multiplier and the extrusion multiplier which was used to generate the g-code by Prusa Slicer.

=> Use your desired **absolut** extrusion multipliers!

Figure: Explicit Flow Rates From 1.04 Down to 0.96 in Steps of 0.01

![Absolut extrusion multipliers](absolut_extrusion_multipliers.png)

Find an example Prusa Slicer project in this (docs) directory.
It is named 'FlowRate.3mf'.

## Prusa Slicer Configuration

### Print Settings - Output Options

#### Output File

Figure: Label Objects

![Label objects](label_objects.png)

#### Post Processing Scripts

Figure: Post Processing Scripts

![Post-processing script](post_processing_script.png)

## Build

* Clone this repository.
* [Install](https://www.rust-lang.org/tools/install) a rust toolchain.
* Run `cargo build --release`
* The executable is in the target/release directory.

## Manual Option

Searching for a solition I also ran over [this Prusa Slicer issue comment](https://github.com/prusa3d/PrusaSlicer/issues/7200#issuecomment-1986715836).
User [@arshish1612](https://github.com/arshish1612) shows a solution which is manual but very flexible.

### Print Settings - Output Options - Other

#### G-Code Substitutions

Figure: Extracted From His Comment

![Manual Option](https://private-user-images.githubusercontent.com/8477844/311409799-4e086cc9-a623-405f-9cd3-5da353695395.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MDk5NzE1NjQsIm5iZiI6MTcwOTk3MTI2NCwicGF0aCI6Ii84NDc3ODQ0LzMxMTQwOTc5OS00ZTA4NmNjOS1hNjIzLTQwNWYtOWNkMy01ZGEzNTM2OTUzOTUucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDMwOSUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAzMDlUMDgwMTA0WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9NmM1MjUxZTI1YWRjYWU0MjUzNTRjNDAxMDJjMGE5YjFhNDVlZjYxYzhkY2M0OTQ0MTg1YTMxMjVmNDcwMWVkYSZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.RR2KtgwMCwJlzn_ELZhiHBXKKyGCLU6PKkKHV-4ynac)

Checking the documentation at [Prusa Slicer](https://help.prusa3d.com/article/g-code-substitutions_301694) I learned that I can achive everything I did with my pre-processor (and more) with the help of these g-code substitutions as well.

Well, too late. Learned some rust, though. :)
