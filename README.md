# mahosorts
Trying to make faster sorting algorithms.

## Mahosort on four:
"Mahosort on four" is a merge-sort like sorting algorithm that is slightly faster than merge-sort when sorting small vectors. 

Functions work like this:

### pub fn sort_on_twos<T: Ord + Copy>(slice: &mut [T]) -> ()
Sorts the _slice_ in groups of two.

Example:

> [9, 7, 1, 2, 6, 5, 9, 11] -> [7, 9, 1, 2, 5, 6, 9, 11]
> 
> Step 1: Look at first two elements of the list. [2, 1]. 2 is bigger than one, so swap them.
> 
> Step 2: Look at third and fourth elements of the list. [4, 3]. 4 is bigger than 3, so swap them.
> 
> Step n: Look at slice[2*n] and slice[2*n+1]. if slice[2*n] is bigger, swap them.
> 
If slice.len() mod 2 != 0, ignores the last element.

### pub fn sort_on_fours<T: Ord + Copy>(slice: &mut [T]) -> ()
Calls sort_on_twos on the _slice_, then sorts the _slice_ in groups of four.

Here is how it works:

> [9, 7, 1, 2, 6, 5, 9, 11] -> [1, 2, 7, 9, 5, 6, 9, 11]
> 
> Step 1: i is an integer in range of 0 and slice.len()/4 (exclusive)
> 
> Step 2: _slice_ is _sorted on twos_ so we know that _slice[4*i]_ < _slice[4*i+1]_ and _slice[4*i+2]_ < _slice[4*i+3]_.
> 
> After sort, _slice[4*i]_ (smallest element in _slice[4*i..=4*i+3]_) can't be _slice[4*i+1]_ or _slice[4*i+3]_,
> 
> so it is going to be min(_slice[4*i]_ or _slice[4*i+2]_).
> 
> _min1_ = min(_slice[4*i]_, _slice[4*i+2]_)
> 
> _max1_ = max(_slice[4*i]_, _slice[4*i+2]_)
> 
> Step 3: We know that _slice[4*i+1]_ > _slice[4*i]_ and _slice[4*i+3]_ > _slice[4*i+2]_. After sort _slice[4*i+3]_
> 
> (biggest element in _slice[4*i..=slice[4*i+3]])_ can't be _slice[4*i]_ or _slice[4*i+2]_ so its going to be max(_slice[4*i+1]_, _slice[4*i+3]_)
> 
> _min2_ = min(_slice[4*i+1]_, _slice[4*i+3]_)
> 
> _max2_ = max(_slice[4*i+1]_, _slice[4*i+3]_)
> 
> Step 4: We know that smallest element is _min1_, and biggest is _max2_.
> 
> _slice[4*i]_ = _min1_
>
> _slice[4*i+3]_ = _max2_
>
> Step 5: _slice[4*i+1]_ and _slice[4*i+2]_ is still not sorted. _slice[4*i+1]_ has to be smaller than _slice[4*i+2]_ so:
> 
> _slice[4*i+1]_ = min(_min2_, _max1_)
> 
> _slice[4*i+2]_ = min(_min2_, _max1_)

If slice.len() mod 4 != 0, ignores the other elements. (last _slice.len() mod 4_ elements.)

### pub fn four_merge<T: Ord + Copy>(vector: &mut Vec<T>, values: [T; 4]) -> ()

Both _vector_ and _values_ must be sorted before.

Merges _vector_ and _values_.

Here is how it works:

> Step 1: We call binary search on _vector_ with _values[0]_, then insert _values[0]_. This is our left limit (_rl_).
>
> We can increase _rl_ by one, because it is the smallest value.
>
> Step 2: We do the same to _values[3]_. This is our right limit (_rr_).
>
> Step 3: We basically do binary search here.

### pub fn mahosort_on_four<T: Ord + Copy>(vector: &mut Vec<T>) -> ()

Sorts the slice in an O(n log n) time complexity algorithm.

Here is how it works:

> Step 1: We call _sort_on_fours_ on vector.
> 
> Step 2: We create a vector: _dummy_ to store sorted vector.
>
> Step 3: We have a pointer called _i_ in range 0 and _vector.len()/4_.
>
> Step 4: We call _four_merge_ with _dummy_ and _vector[4*i]_, _vector[4*i+1]_, _vector[4*i+2]_, _vector[4*i+3]_.
>
> If vector.len() mod 4 != 0, we add last _vector.len() mod 4_ elements with binary search.






