--Dance and Movement Classes

-- Class definition
local DanceClass = {}

-- Constructor
function DanceClass:new(duration, style, instructor)
  local obj = {
    duration = duration,
    style = style,
    instructor = instructor
  }
  self.__index = self
  return setmetatable(obj, self)
end

-- Setters and getters
function DanceClass:getDuration()
  return self.duration
end

function DanceClass:setDuration(duration)
  self.duration = duration
end

function DanceClass:getStyle()
  return self.style
end

function DanceClass:setStyle(style)
  self.style = style
end

function DanceClass:getInstructor()
  return self.instructor
end

function DanceClass:setInstructor(instructor)
  self.instructor = instructor
end

-- Start class
function DanceClass:start()
  print('Starting '..self:getDuration()..' minutes '..self:getStyle()..' class with '..self:getInstructor())
end

-- End class
function DanceClass:end()
  print('Ending '..self:getDuration()..' minutes '..self:getStyle()..' class with '..self:getInstructor())
end

-- List of classes
local classes = {
  DanceClass:new(60, 'Jazz', 'Jane'),
  DanceClass:new(45, 'Hip Hop', 'John'),
  DanceClass:new(30, 'Tap', 'Sam'),
  DanceClass:new(45, 'Ballet', 'Lily')
}

-- Loop through each class and start and end it
for _, cls in ipairs(classes) do
  cls:start()
  cls:end()
end