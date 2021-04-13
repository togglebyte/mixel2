Application 
    -> Input
        -> Combine char and Key into one state. Clear all on key release
    -> CommandLine
    -> Canvas


CommandLine
    execute() -> Vec<Commands> -> Canvas
    input() only chars and enter.
    quit() application


Canvas
    execute(commands, undo_stack)
    draw(x, y)
    read(x, y)
    input() -> (operator + motion) -> apply to canvas
    undo(undo_stack)
    forward / backward?


Operators:
* Draw(x, y)
* Delete(x, y)
* Yank(from_xy, to_xy)

Motions:
* NextPixelOfSameColour


Operator + Motion:
* Layer
* Pixel buffer access
