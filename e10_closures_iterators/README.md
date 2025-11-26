## Common iterator adapters (with examples)

_Adapters are lazy transforms returning new iterators._

`map` — transform each item

`filter` — keep items that match predicate

`filter_map` — map and drop None

`flat_map` — map to iterator then flatten

`enumerate` — yield (index, item)

`zip` — pair two iterators

`chain` — sequentially combine

`take`, `skip`, `take_while`, `skip_while` — limiting

`inspect` — side-effectful peek (debugging)

`peekable()` — get a Peekable adapter

`scan` — stateful transform

`fuse` — stops yielding after None