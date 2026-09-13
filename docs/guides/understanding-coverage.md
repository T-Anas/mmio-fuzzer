# Understanding coverage

Coverage is an AFL-style bitmap of PC transitions.
New edges mean new behaviour; the engine keeps the input that produced them.
Stagnation over a long run means a tight loop and ends the execution.
