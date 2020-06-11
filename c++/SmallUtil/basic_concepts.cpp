/*template<class It,class Entry>
concept HolderOfOne =
requires(It v) {
	{v.val()}->Entry,

};
template<class It,class One,class Two>
concept Pair =
requires(It v) {
	{v.first()}->One,
	{ v.second() }->Two,

};
template<class It>
concept Infinite =
requires(It v) {
	{v.isInfinite()} -> true,

};
template<class It>
concept Number = std::is_integral<It>::value || std::is_floating_point<It>::value || Infinite<It>;*/