//
// Created by imagifight on 4/21/24.
//

#ifndef QUEUES_HPP
#define QUEUES_HPP
#include <optional>

#include "vulkan.h"

struct QueueFamilyIndices {
    std::optional<uint32_t> graphicsFamily;

    [[nodiscard]] bool isComplete() const {
        return graphicsFamily.has_value();
    }
};

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device);

#endif //QUEUES_HPP
