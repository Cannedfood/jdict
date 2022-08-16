#pragma once

#include <mutex>
#include <deque>
#include <functional>

namespace jdict {

template<class Key, class Value>
class cache {
	std::mutex lock;
	struct Entry {
		Key key;
		Value value;
	};
	std::deque<Entry> entries;
	size_t maxEntries;
public:
	cache(size_t maxEntries = 1024) :
		maxEntries(maxEntries)
	{}

	void clear() { entries.clear(); }

	Value get_or_create(Key key, std::function<Value()> create) {
		{
			std::lock_guard _(lock);
			for(auto& entry : entries) {
				if(entry.key == key)
					return entry.value;
			}
		}

		auto result = create();

		{
			std::lock_guard _(lock);
			entries.push_back({
				.key = key,
				.value = result
			});
			if(entries.size() > maxEntries)
				entries.pop_front();
		}

		return result;
	}
};

} // namespace jdict
