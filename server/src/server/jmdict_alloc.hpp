#pragma once

#include <algorithm>
#include <bit>
#include <cassert>
#include <span>
#include <string_view>

namespace jdict {

class arena_allocator {
	constexpr static unsigned MaxAlign = 32;

	struct block {
		block* previous_block;
		char*  head;
		char*  end;
		alignas(MaxAlign) char data[];
	};
	block*   current_block = nullptr;
	unsigned new_block_size;
public:
	arena_allocator(unsigned block_size = 8096) noexcept :
		new_block_size(block_size)
	{}
	~arena_allocator() noexcept { clear(); }

	void* alloc(size_t nbytes, size_t align) noexcept {
		assert(align < MaxAlign);

		[[unlikely]]
		if(!current_block) new_block(nbytes);

		char* newHead = next_aligned_position(current_block->head, align) + nbytes;
		assert(((size_t)newHead) % align == 0);

		[[likely]]
		if(newHead <= current_block->end) {
			char* result = current_block->head;
			current_block->head = newHead;
			return result;
		}
		else {
			new_block(nbytes);

			char* result = next_aligned_position(current_block->head, align);
			current_block->head = result + nbytes;
			return result;
		}
	}
	template<class T>
	T* alloc(size_t n) noexcept {
		constexpr size_t size = std::max(alignof(T), sizeof(T));
		return reinterpret_cast<T*>(alloc(size, alignof(T)));
	}
	std::string_view alloc(std::string_view text) noexcept {
		char* data = alloc<char>(text.size());
		std::copy_n(text.data(), text.size(), data);
		return std::string_view(data, text.size());
	}

	void clear() noexcept {
		block* p = current_block;
		while(p) {
			block* d = p;
			p = d->previous_block;
			free(d);
		}
		current_block = nullptr;
	}

private:
	inline char* next_aligned_position(char* memory, size_t alignment) {
		assert(std::has_single_bit(alignment)); // Only works with powers of 2
		return (char*)( (((size_t) memory) + alignment - 1) & -alignment );

		// General version, for powers other than 2
		// return (char*)(((((size_t) memory) + alignment - 1)) / alignment * alignment);
	}

	void new_block(unsigned minSize) noexcept {
		auto capacity = std::max(minSize, new_block_size);

		auto* b = (block*) malloc(sizeof(block) + capacity);
		b->head = b->data;
		b->end  = b->head + capacity;
		b->previous_block = current_block;

		current_block = b;
	}
};

} // namespace jdict
