p = require 'libplunder'

samp = p.ofWav '/home/admin1234/Kikuo - あなぐらぐらし [I15sK7dNMOM].wav'

sheet = [[
        | [      ]
 samp   | o
]]

conf = { interval = 1000 }

p.p1.render(conf, sheet, { samp = samp })
