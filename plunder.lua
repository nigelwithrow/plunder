--!strict
local libplunder = require 'libplunder'

local plunder = {}

plunder.p1 = libplunder.p1
-- plunder.midi1 = libplunder.midi1
plunder.ofWav = libplunder.of_wav

-- function plunder.p1:instruments(instrs)
--   local i = 0
--   for k, v in pairs(instrs) do i = i + 1 end
--   print(i .. " instruments attached")
--   return self
-- end

-- ---@param sheet string
-- function plunder.p1.sheet(self, sheet)
--   print("> Sheet attached")
--   print(sheet:match('\n')[1])
--   print("26 + 13 = " .. libplunder.add(26, 13))
--   print("< Sheet attached")
--   return self
-- end

-- function plunder.ofWav(filename)
--   return {
--     filename,
--     play = function() return 0 end,
--     stretch = function (self, factor)
--       print("Stretched by a factor of: " .. factor)
--       return self
--     end,
--   }
-- end

function plunder.midi(filename)
  return {
    notes = function(self, interval)
      print("Note interval attached: " .. interval)
      return self
    end,
    filename,
    play = function()
      return 0
    end,
  }
end

function plunder.global()
  _G.p1 = plunder.p1
  _G.ofWav = plunder.ofWav
  _G.midi = plunder.midi
  _G.midi1 = plunder.midi1
  return plunder
end

return plunder
