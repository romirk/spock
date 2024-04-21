//
// Created by imagifight on 4/21/24.
//

#ifndef UTILS_HPP
#define UTILS_HPP
#include <vector>

#include "vulkan.h"

struct SwapChainSupportDetails {
    VkSurfaceCapabilitiesKHR capabilities;
    std::vector<VkSurfaceFormatKHR> formats;
    std::vector<VkPresentModeKHR> presentModes;
};

void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT &createInfo);




#endif //UTILS_HPP
