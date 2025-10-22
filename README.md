# k-bonacci
Rust iterator to generate generalised Fibonacci sequences

it's initialised with KBonacci::new(k) , where k is the amount of previous terms summed. 
it uses BigUint for arbritrary precision

there's a cool instance in examples/demo.rs where each sequence is skipped to the point is is unique from the previous,
because k-bonacci sequences just gives powers of 2 until 2k-1, after which they diverge. 
