local a = {
  hey = function() print('a says hello') end,
  there = function() print('a says over there') end
}

setmetatable(a, {__call = function(self, name)
  print('a says hello to ' .. name)
end})

local b = {
  a = a,
  foo = function() print("b's foo") end,
}

setmetatable(b, {
  __index = function (self, key)
    return self.a[key]
  end,
  __call = function(self, ...)
    return self.a(...)
  end,
})

b.hey()
b("nigel")
