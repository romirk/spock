//
// Created by imagifight on 4/21/24.
//

#ifndef UTILS_HPP
#define UTILS_HPP
#include "vulkan.h"

void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT &createInfo);

uint32_t rateDeviceSuitability(VkPhysicalDevice device);

#endif //UTILS_HPP
