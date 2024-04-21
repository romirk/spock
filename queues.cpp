//
// Created by imagifight on 4/21/24.
//

#include "queues.hpp"

#include <vector>


// ReSharper disable once CppParameterMayBeConst
QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device) {
    QueueFamilyIndices indices;

    uint32_t queueFamilyCount = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, nullptr);

    std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, queueFamilies.data());

    int i = 0;
    for (const auto &[queueFlags, queueCount, timestampValidBits, minImageTransferGranularity]: queueFamilies) {
        if (queueFlags & VK_QUEUE_GRAPHICS_BIT)
            indices.graphicsFamily = i++;
        if (indices.isComplete())
            break;
    }
    return indices;
}
