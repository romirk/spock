//
// Created by imagifight on 4/21/24.
//

#ifndef QUEUES_HPP
#define QUEUES_HPP
#include <optional>

#include "vulkan.h"

struct QueueFamilyIndices {
    std::optional<uint32_t> graphicsFamily;
    std::optional<uint32_t> presentFamily;

    [[nodiscard]] bool isComplete() const {
        return graphicsFamily.has_value() && presentFamily.has_value();
    }
};

#endif //QUEUES_HPP
