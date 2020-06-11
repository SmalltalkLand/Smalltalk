#include "./base_calasses.cpp"
#include <elfio/elfio.hpp>  
#include <vector>
class ElfNoCode: public std::exception{};
class ElfReturn {
public:
	int code;
	bool restartWithElf;

};
class ElfExecutor {
	ELFIO::elfio reader;
public:
	ElfExecutor(std::ifstream s) { reader = new ELFIO::elfio(); reader.load(s); };
	~ElfExecutor() { delete reader; };
	void execute(std::vector<void*()(void*)> syscalls) {
		bool r = true;
		while (r) {
			void* data = syscalls.data();
			ELFIO::Elf_Half size = readers.sections.size();
			ELFIO::section* s = 0;
			for (int i = 0; i < sec_num; ++i) {
				s = reader.sections[i];
				if (s->get_name() == ".st-elf-code")break;
			};
			if (s == 0) { throw new ElfNoCode(); };
			void* (sdata)(void*) = s->get_data();
			ElfReturn rreturn = sdata(data);
			r = rreturn.restartWithElf;
		};
	};
}; 