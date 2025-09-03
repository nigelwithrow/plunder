require './plunder'.global()

return p1
.new

:sheet [[
         | [              ]
  kick   | o   o   o   o
  snare  |   "   "   "   "
  hihat  | ''''''''''''''''
  piano  | (              )
  sample | 
         |
]]

:instruments {
  kick  = samp 'kick.wav',
  snare = samp 'snare.wav',
  hihat = samp 'hihat.wav',
  piano =
    p1
    .new
    :sheet [[
        [                            ]
        A2      F2      C3      G2
        A5  C5  G5  C5  A5  C5  G5  C5  
    ]]
    :instruments { midi 'TimGM6mb.sf2' :notes (4) },

  sample = samp 'on-the-floor.wav'
}
