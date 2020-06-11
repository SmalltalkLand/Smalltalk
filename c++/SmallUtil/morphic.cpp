#include "./base_classes.cpp"
#include <vector>
class Canvas {
	 virtual void drawLine(TwoOf<long, long>, TwoOf<long, long>, TwoOf<int, TwoOf<int, int>>) = 0;
	 virtual void drawRect(TwoOf<long, long>, TwoOf<long, long>, TwoOf<int, TwoOf<int, int>>) = 0;
};
class Morph {
public:
	std::vector<s* (Morph)> children;
	virtual void drawOn(s* (Canvas) c) = 0;
	void fullDrawOn(s* (Canvas)c) {
		this.drawOn(c);
		for (int i = 0; i <= this.children.size(); i++) {
			this.children[i].fullDrawOn(c);
		};
	};
};
class PositionedMorph: public Morph {
public:
	s* (TwoOf<long, long>) position;

};
class ColoredMorph : public PositionedMorph {
public:
	s* (TwoOf<int, TwoOf<int, int>>) color;

};
class ShadowMorph: protected Morph{};
class ShadowPositionedMorph: protected PositionedMorph{};
class ShadowColoredMorph: protected ColoredMorph{};

class WorldMorph : public Morph {


};
class HandleMorph : public Morph {
public:
	int handle;
	static virtual void draw(int, s* (Canvas)) = 0;
	void drawOn(s* (Canvas)c) {
		draw(handle, c);
	}
};