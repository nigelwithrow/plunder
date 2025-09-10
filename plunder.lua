--!strict
local libplunder = require 'libplunder'

local plunder = {}


--
-- ofWav
--
---@class Instrument
---@type fun(path: string): Instrument
plunder.ofWav = libplunder.ofWav


--
-- p1
--

---@class P1Config: {interval: number}
---@alias P1InstrumentMap ({[string]: Instrument} | Instrument[])

---@class P1: {_conf: P1Config?; _sheet: string?; _instruments: P1InstrumentMap, _buffer: Instrument?}
---@field sheet fun(self: P1, sheet: string): P1
---@field instruments fun(self: P1, instruments: P1InstrumentMap): P1
---@field get {sheet: fun(P1): string; instruments: fun(P1): ({[string]: Instrument} | Instrument[])}

local p1 = {}
p1.__metatable = {}
p1.__metatable.__index = p1.__metatable
---@param self P1
---@param sheet string
---@return P1
function p1.__metatable:sheet(sheet)
  self._sheet = sheet
  if self._conf and self._sheet and self._instruments then
    self._buffer = libplunder.p1.render(self._conf, self._sheet, self._instruments)
  end
  return self
end

---@param self P1
---@param instruments P1InstrumentMap
---@return P1
function p1.__metatable:instruments(instruments)
  self._instruments = instruments
  if self._conf and self._sheet and self._instruments then
    self._buffer = libplunder.p1.render(self._conf, self._sheet, self._instruments)
  end
  return self
end

---@param conf P1Config
---@return P1
p1.new = function(conf)
  local self = setmetatable({ _conf = conf }, p1.__metatable)
  return self
end

plunder.p1 = p1

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
