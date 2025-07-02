#pragma once

#include "../data_container/data_container.h"
#include <essentia/pool.h>
#include <rust/cxx.h>
#include <vector>

namespace essentia_bridge {
struct StereoSample;

struct DataContainer;
enum class DataType : std::uint8_t;

class PoolBridge {
public:
  PoolBridge();
  PoolBridge(essentia::Pool &pool);
  ~PoolBridge();

  essentia::Pool into_pool();

  std::unique_ptr<PoolBridge> clone() const;

  void set(rust::Str key, std::unique_ptr<DataContainer> data_container);
  std::unique_ptr<DataContainer> get(rust::Str key) const;
  bool contains(rust::Str key) const;
  rust::Vec<rust::String> keys() const;

private:
  essentia::Pool *_pool;
  bool _is_owner;
};

std::unique_ptr<PoolBridge> create_pool_bridge();

} // namespace essentia_bridge