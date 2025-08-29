
return
p1

.instruments {
  kick  = samp 'kick.wav',
  snare = samp 'snare.wav',
  hihat = samp 'hihat.wav',
  piano =
    p1
    .instruments { midi 'TimGM6mb.sf2' .notes (4) }
    .sheet [[
        [                            ]
        A2      F2      C3      G2
        A5  C5  G5  C5  A5  C5  G5  C5  
    ]]
}

.sheet [[
         | [              ]
  kick   | o   o   o   o   
  snare  |   "   "   "   "
  hihat  | ''''''''''''''''
  piano  | (              )
         |
]]
