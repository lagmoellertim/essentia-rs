#include "pool_bridge.h"
#include "../variant_data/variant_data.h"
#include <algorithm> // For std::find
#include <essentia/pool.h>
#include <stdexcept>
#include <variant>

namespace essentia_bridge {

// --- Helper Visitor for PoolBridge::set ---
struct SetVisitor {
  essentia::Pool &pool;
  const std::string &key;

  // Supported types
  void operator()(const std::string &value) { pool.set(key, value); }
  void operator()(float value) {
    pool.set(key, static_cast<essentia::Real>(value));
  }
  void operator()(const std::vector<std::string> &value) {
    pool.set(key, value);
  }
  void operator()(const std::vector<float> &value) {
    static_assert(std::is_same_v<float, essentia::Real>,
                  "essentia::Real should be float");
    const auto &real_vector =
        reinterpret_cast<const std::vector<essentia::Real> &>(value);
    pool.set(key, real_vector);
  }
  // Note: Tensor<Real> is not directly in VariantData, but might be added
  // later. void operator()(const essentia::Tensor<essentia::Real> &value) {
  // pool.set(key, value); }

  // Overloads for all other types in the variant to make the visitor exhaustive
  template <typename T> void operator()(const T &) {
    throw std::runtime_error("Unsupported data type for Pool::set");
  }
};

// --- PoolBridge Implementation ---

// Owning constructor
PoolBridge::PoolBridge() : _pool(new essentia::Pool()), _is_owner(true) {}

// Non-owning constructor
PoolBridge::PoolBridge(essentia::Pool &pool) : _pool(&pool), _is_owner(false) {}

// Destructor
PoolBridge::~PoolBridge() {
  if (_is_owner) {
    delete _pool;
  }
}

// into_pool method
essentia::Pool PoolBridge::into_pool() {
  if (!_is_owner) {
    // If we are not the owner, we cannot move the pool.
    // We can only return a copy.
    return essentia::Pool(*_pool);
  }
  essentia::Pool moved_pool = std::move(*_pool);
  delete _pool;
  _pool = nullptr;
  _is_owner = false;
  return moved_pool;
}

// clone method
std::unique_ptr<PoolBridge> PoolBridge::clone() const {
  auto new_bridge =
      std::make_unique<PoolBridge>(); // Creates a new owning bridge

  // TODO check if this works
  new_bridge->_pool->merge(*this->_pool, "replace"); // Copy contents
  return new_bridge;
}

void PoolBridge::set(rust::Str key, std::unique_ptr<VariantData> variant_data) {
  if (!_pool) {
    throw std::runtime_error("Pool pointer is null and cannot be used");
  }
  std::string cpp_key(key);
  std::visit(SetVisitor{*_pool, cpp_key}, variant_data->data);
}

std::unique_ptr<VariantData> PoolBridge::get(rust::Str key) const {
  if (!_pool) {
    throw std::runtime_error("Pool pointer is null and cannot be used");
  }
  std::string cpp_key(key);
  if (_pool->contains<essentia::Real>(cpp_key)) {
    return std::make_unique<VariantData>(_pool->value<essentia::Real>(cpp_key));
  }
  if (_pool->contains<std::string>(cpp_key)) {
    return std::make_unique<VariantData>(_pool->value<std::string>(cpp_key));
  }
  if (_pool->contains<std::vector<essentia::Real>>(cpp_key)) {
    return std::make_unique<VariantData>(
        _pool->value<std::vector<essentia::Real>>(cpp_key));
  }
  if (_pool->contains<std::vector<std::string>>(cpp_key)) {
    return std::make_unique<VariantData>(
        _pool->value<std::vector<std::string>>(cpp_key));
  }
  // Note: Add other supported types from essentia::Pool here

  throw std::runtime_error(
      "Key '" + cpp_key +
      "' not found in Pool or has an unsupported type for get().");
}

bool PoolBridge::contains(rust::Str key) const {
  if (!_pool) {
    return false;
  }
  const std::string key_str(key);
  const auto &names = _pool->descriptorNames();
  return std::find(names.begin(), names.end(), key_str) != names.end();
}

rust::Vec<rust::String> PoolBridge::keys() const {
  if (!_pool) {
    return rust::Vec<rust::String>();
  }
  const std::vector<std::string> &cpp_keys = _pool->descriptorNames();
  rust::Vec<rust::String> rust_keys;
  for (const auto &key : cpp_keys) {
    rust_keys.push_back(rust::String(key));
  }
  return rust_keys;
}

// --- Factory Function ---
std::unique_ptr<PoolBridge> create_pool_bridge() {
  return std::make_unique<PoolBridge>();
}

} // namespace essentia_bridge