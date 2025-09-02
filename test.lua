a = { __tostring = function() return 'aaa' end, message = function(foo) print(foo) print(self) end }
setmetatable(a, a)

a.message(8)
