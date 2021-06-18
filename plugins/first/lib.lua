Lib = {}

function Lib.thing ()
    setColor(0, 0, 155)

    for x = 5, 22 do 
        for y = 5, 22 do
            putPixel(x, y)
        end
    end

    setColor(0, 0, 0)

    for x = 5, 22 do 
        putPixel(x, 5)
    end

end
