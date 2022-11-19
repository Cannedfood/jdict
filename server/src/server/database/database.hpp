#pragma once

#include <cassert>
#include <cstdint>
#include <vector>
#include <limits>

namespace jdict {

template<class DiffT, class T>
struct relptr {
	constexpr static uint32_t step = alignof(relptr) < alignof(T)? alignof(relptr) : alignof(T);

	DiffT offset = 0;

	relptr() = default;
	relptr(T* ptr) noexcept { set(ptr); }

	void set(T* ptr) {
		if(ptr == nullptr)
			offset = 0;
		else {
			std::ptrdiff_t o = (reinterpret_cast<std::byte*>(ptr) - reinterpret_cast<std::byte*>(this)) / step;
			assert(o >= std::numeric_limits<DiffT>::min() && o <= std::numeric_limits<DiffT>::max());
			offset = o;
		}
	}
	T* get() const {
		if(offset == 0)
			return nullptr;
		else
			return reinterpret_cast<T*>(reinterpret_cast<std::byte>(this) + offset * step);
	}

	T& operator*() const { return *get(); }
	T* operator->() const { return get(); }

	operator T*() const { return get(); }
};

template<class SizeT, class T>
struct inline_span {
	SizeT size;
	T     data[];
};

template<class SizeT, class KeyT, class ValueT>
using inline_map = inline_span<SizeT, std::pair<KeyT, ValueT>>;

struct database_jmdict {
	struct kanji_t {};
	struct reading_t {};
	struct sense_t {};
	struct entry_t {};

	const char magic[8] = "jmdict";
	uint32_t   version  = 1;

};

} // namespace jdict
