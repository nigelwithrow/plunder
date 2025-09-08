require 'plunder'.global()

-- TODO: add specification in p1 sheets to be able to parse both audio & midi
return p1.new

-- :config {
--   next = {
--     init = {}, -- no ix to stream
--     match = {
--       ['[])]'] = function(s) return {} end,
--       ['[^ ]'] = function() return { ofWav.ISET.NEXT } end,
--       [' '] = function(s) return s end,
--     }
--   }
-- }

:sheet [[
         | [              ]
  kick   | o   o   o   o
  snare  |   "   "   "   "
  hihat  | ''''''''''''''''
  sample | [      ][      ]
         |
]]

:instruments {
  kick  = ofWav 'kick.wav',
  snare = ofWav 'snare.wav',
  hihat = ofWav 'hihat.wav',
  sample = ofWav 'on-the-floor.wav' :stretch (2)
}
