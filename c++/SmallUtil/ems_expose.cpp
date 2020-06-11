#include <string>
#include "./base_classes.cpp"
#include <emscripten/emscripten.h>
class EmAble {
public:
	abstract virtual void run(int argc, char** argv);
	static std::vector <TwoOf<std::string, EmAble>> map;
};
extern "C" {
	void EMSCRIPTEN_KEEPALIVE emRunAble(int argc, char** argv) {
		char* arg1 = argv[0];
		std::string s = arg1;
		for (int i = 0; i <= EmAble::map.size(); i++) {
			TwoOf<std::string, EmAble> e = EmAble::map[i];
			std::string first = e.first();
			if (first.compare(s) == = 0) {
				EmAble second = e.second();
				second.run(argc - 1, argv + 1);
				break;
			};
		};
	}
}