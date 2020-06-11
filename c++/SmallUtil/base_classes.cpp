#include "./basic_concepts.cpp"
#include <memory>
#define s*(val) std::shared_ptr<val>
template<class T>
class Awaitable {
	abstract void await(void(*v)(T));

};
class Infinity {
	bool isInfinite() { return true; }

};
template<class T>
class OneOf {
public:
	T val_;
	T val() { return val_; }

};
template<class O,class T>
class TwoOf {
public:
	O first_;
	T second_;
	O first() { return first_; };
	T second() { return second_; }

};