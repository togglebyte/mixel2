Lib = {}

function Lib.thing ()

    for x = 31, 0, -1 do 
        for y = 31, 0, -1 do
            setColor(255, 0, 0)
            putPixel(x, y)
        end
    end

end
