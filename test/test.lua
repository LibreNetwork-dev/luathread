local luathread = require("luathread")

luathread.spawn(function() 
    for i = 1, 100 do 
        print("hi")
    end
end)

print("main ended")

os.execute("sleep 1")  