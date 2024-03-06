#pragma once

#include "avm_common.hpp"

namespace bb::avm_trace {

void log_avm_trace(std::vector<Row> const& trace, size_t beg, size_t end, bool enable_selectors = false);

} // namespace bb::avm_trace