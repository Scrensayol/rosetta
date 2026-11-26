-- middleclass 4.1.1 (kikito/middleclass 359f0e2): classes, subclasses, mixins and every
-- metamethod Luau has, built from metatables and __index chains. Upstream's specs, run
-- unchanged except where Luau differs from Lua 5.1 (see libraries/README.md)
local class = require("middleclass")

-- class_spec.lua
do
describe('class()', function()

  describe('when given no params', function()
    it('it throws an error', function()
      assert.error(class)
    end)
  end)

  describe('when given a name', function()
    it('the resulting class has the correct name and Object as its superclass', function()
      local TheClass = class('TheClass')
      assert.equal(TheClass.name, 'TheClass')
      assert.is_nil(TheClass.super)
    end)
  end)

  describe('when given a name and a superclass', function()
    it('the resulting class has the correct name and superclass', function()
      local TheSuperClass = class('TheSuperClass')
      local TheSubClass = class('TheSubClass', TheSuperClass)
      assert.equal(TheSubClass.name, 'TheSubClass')
      assert.equal(TheSubClass.super, TheSuperClass)
    end)
  end)

end)
end

-- classes_spec.lua
do
describe('A Class', function()

  describe('Default stuff', function()

    local AClass

    before_each(function()
      AClass = class('AClass')
    end)

    describe('name', function()
      it('is correctly set', function()
        assert.equal(AClass.name, 'AClass')
      end)
    end)

    describe('tostring', function()
      it('returns "class *name*"', function()
        assert.equal(tostring(AClass), 'class AClass')
      end)
    end)

    describe('()', function()
      it('returns an object, like Class:new()', function()
        local obj = AClass()
        assert.equal(obj.class, AClass)
      end)
    end)

    describe('include', function()
      it('throws an error when used without the :', function()
        assert.error(function() AClass.include() end)
      end)
      it('throws an error when passed a non-table:', function()
        assert.error(function() AClass:include(1) end)
      end)
    end)

    describe('subclass', function()

      it('throws an error when used without the :', function()
        assert.error(function() AClass.subclass() end)
      end)

      it('throws an error when no name is given', function()
        assert.error( function() AClass:subclass() end)
      end)

      describe('when given a subclass name', function()

        local SubClass

        before_each(function()
          function AClass.static:subclassed(other) self.static.child = other end
          SubClass = AClass:subclass('SubClass')
        end)

        it('it returns a class with the correct name', function()
          assert.equal(SubClass.name, 'SubClass')
        end)

        it('it returns a class with the correct superclass', function()
          assert.equal(SubClass.super, AClass)
        end)

        it('it invokes the subclassed hook method', function()
          assert.equal(SubClass, AClass.child)
        end)

        it('it includes the subclass in the list of subclasses', function()
          assert.is_true(AClass.subclasses[SubClass])
        end)

      end)

    end)

  end)



  describe('attributes', function()

    local A, B

    before_each(function()
      A = class('A')
      A.static.foo = 'foo'

      B = class('B', A)
    end)

    it('are available after being initialized', function()
      assert.equal(A.foo, 'foo')
    end)

    it('are available for subclasses', function()
      assert.equal(B.foo, 'foo')
    end)

    it('are overridable by subclasses, without affecting the superclasses', function()
      B.static.foo = 'chunky bacon'
      assert.equal(B.foo, 'chunky bacon')
      assert.equal(A.foo, 'foo')
    end)

  end)

  describe('methods', function()

    local A, B

    before_each(function()
      A = class('A')
      function A.static:foo() return 'foo' end

      B = class('B', A)
    end)

    it('are available after being initialized', function()
      assert.equal(A:foo(), 'foo')
    end)

    it('are available for subclasses', function()
      assert.equal(B:foo(), 'foo')
    end)

    it('are overridable by subclasses, without affecting the superclasses', function()
      function B.static:foo() return 'chunky bacon' end
      assert.equal(B:foo(), 'chunky bacon')
      assert.equal(A:foo(), 'foo')
    end)

  end)

end)
end

-- default_methods_spec.lua
do
describe('Default methods', function()
  local Object
  before_each(function()
    Object = class('Object')
  end)

  describe('name', function()
    it('is correctly set', function()
      assert.equal(Object.name, 'Object')
    end)
  end)

  describe('tostring', function()
    it('returns "class Object"', function()
      assert.equal(tostring(Object), 'class Object')
    end)
  end)

  describe('()', function()
    it('returns an object, like Object:new()', function()
      local obj = Object()
      assert.is_true(obj:isInstanceOf(Object))
    end)
  end)

  describe('subclass', function()

    it('throws an error when used without the :', function()
      assert.error(function() Object.subclass() end)
    end)

    it('throws an error when no name is given', function()
      assert.error( function() Object:subclass() end)
    end)

    describe('when given a class name', function()

      local SubClass

      before_each(function()
        SubClass = Object:subclass('SubClass')
      end)

      it('it returns a class with the correct name', function()
        assert.equal(SubClass.name, 'SubClass')
      end)

      it('it returns a class with the correct superclass', function()
        assert.equal(SubClass.super, Object)
      end)

      it('it includes the subclass in the list of subclasses', function()
        assert.is_true(Object.subclasses[SubClass])
      end)

    end)

  end)

  describe('instance creation', function()

    local SubClass

    before_each(function()
      SubClass = class('SubClass')
      function SubClass:initialize() self.mark=true end
    end)

    describe('allocate', function()

      it('allocates instances properly', function()
        local instance = SubClass:allocate()
        assert.equal(instance.class, SubClass)
        assert.equal(tostring(instance), "instance of " .. tostring(SubClass))
      end)

      it('throws an error when used without the :', function()
        assert.error(Object.allocate)
      end)

      it('does not call the initializer', function()
        local allocated = SubClass:allocate()
        assert.is_nil(allocated.mark)
      end)

      it('can be overriden', function()

        local previousAllocate = SubClass.static.allocate

        function SubClass.static:allocate()
          local instance = previousAllocate(SubClass)
          instance.mark = true
          return instance
        end

        local allocated = SubClass:allocate()
        assert.is_true(allocated.mark)
      end)

    end)

   describe('new', function()

      it('initializes instances properly', function()
        local instance = SubClass:new()
        assert.equal(instance.class, SubClass)
      end)

      it('throws an error when used without the :', function()
        assert.error(SubClass.new)
      end)

      it('calls the initializer', function()
        local initialized = SubClass:new()
        assert.is_true(initialized.mark)
      end)

    end)

    describe('isInstanceOf', function()

      describe('primitives', function()
        local o = Object:new()
        local primitives = {nil, 1, 'hello', {}, function() end, Object:new()}

        describe('used as classes', function()
          for _,primitive in pairs(primitives) do
            local theType = type(primitive)
            it('object:isInstanceOf(, '.. theType ..') returns false', function()
              assert.is_falsy(o:isInstanceOf(primitive))
            end)
          end
        end)

        describe('used as instances', function()
          for _,primitive in pairs(primitives) do
            local theType = type(primitive)
            it('Object.isInstanceOf('.. theType ..', Object) returns false without error', function()
              assert.is_falsy(Object.isInstanceOf(primitive, Object))
            end)
          end
        end)


      end)

      describe('An instance', function()
        local Class1 = class('Class1')
        local Class2 = class('Class2', Class1)
        local Class3 = class('Class3', Class2)
        local UnrelatedClass = class('Unrelated')

        local o1, o2, o3 = Class1:new(), Class2:new(), Class3:new()

        it('isInstanceOf its class', function()
          assert.is_true(o1:isInstanceOf(Class1))
          assert.is_true(o2:isInstanceOf(Class2))
          assert.is_true(o3:isInstanceOf(Class3))
        end)

        it('is instanceOf its class\' superclasses', function()
          assert.is_true(o2:isInstanceOf(Class1))
          assert.is_true(o3:isInstanceOf(Class1))
          assert.is_true(o3:isInstanceOf(Class2))
        end)

        it('is not instanceOf its class\' subclasses', function()
          assert.is_false(o1:isInstanceOf(Class2))
          assert.is_false(o1:isInstanceOf(Class3))
          assert.is_false(o2:isInstanceOf(Class3))
        end)

        it('is not instanceOf an unrelated class', function()
          assert.is_false(o1:isInstanceOf(UnrelatedClass))
          assert.is_false(o2:isInstanceOf(UnrelatedClass))
          assert.is_false(o3:isInstanceOf(UnrelatedClass))
        end)

      end)

    end)

  end)

  describe('isSubclassOf', function()

    it('returns false for instances', function()
      assert.is_false(Object:isSubclassOf(Object:new()))
    end)

    describe('on primitives', function()
      local primitives = {nil, 1, 'hello', {}, function() end}

      for _,primitive in pairs(primitives) do
        local theType = type(primitive)
        it('returns false for ' .. theType, function()
          assert.is_false(Object:isSubclassOf(primitive))
        end)
      end

    end)

    describe('Any class (except Object)', function()
      local Class1 = class('Class1')
      local Class2 = class('Class2', Class1)
      local Class3 = class('Class3', Class2)
      local UnrelatedClass = class('Unrelated')

      it('is subclassOf its direct superclass', function()
        assert.is_true(Class2:isSubclassOf(Class1))
        assert.is_true(Class3:isSubclassOf(Class2))
      end)

      it('is subclassOf its ancestors', function()
        assert.is_true(Class3:isSubclassOf(Class1))
      end)

      it('is a subclassOf its class\' subclasses', function()
        assert.is_true(Class2:isSubclassOf(Class1))
        assert.is_true(Class3:isSubclassOf(Class1))
        assert.is_true(Class3:isSubclassOf(Class2))
      end)

      it('is not a subclassOf an unrelated class', function()
        assert.is_false(Class1:isSubclassOf(UnrelatedClass))
        assert.is_false(Class2:isSubclassOf(UnrelatedClass))
        assert.is_false(Class3:isSubclassOf(UnrelatedClass))
      end)

    end)
  end)
end)
end

-- instances_spec.lua
do
describe('An instance', function()

  describe('attributes', function()

    local Person

    before_each(function()
      Person = class('Person')
      function Person:initialize(name)
        self.name = name
      end
    end)

    it('are available in the instance after being initialized', function()
      local bob = Person:new('bob')
      assert.equal(bob.name, 'bob')
    end)

    it('are available in the instance after being initialized by a superclass', function()
      local AgedPerson = class('AgedPerson', Person)
      function AgedPerson:initialize(name, age)
        Person.initialize(self, name)
        self.age = age
      end

      local pete = AgedPerson:new('pete', 31)
      assert.equal(pete.name, 'pete')
      assert.equal(pete.age, 31)
    end)

  end)

  describe('methods', function()

    local A, B, a, b

    before_each(function()
      A = class('A')
      function A:overridden() return 'foo' end
      function A:regular() return 'regular' end

      B = class('B', A)
      function B:overridden() return 'bar' end

      a = A:new()
      b = B:new()
    end)

    it('are available for any instance', function()
      assert.equal(a:overridden(), 'foo')
    end)

    it('are inheritable', function()
      assert.equal(b:regular(), 'regular')
    end)

    it('are overridable', function()
      assert.equal(b:overridden(), 'bar')
    end)

  end)

end)
end

-- metamethods_spec.lua
do
describe('Metamethods', function()
  describe('Custom Metamethods', function()
    local Vector, v, w
    before_each(function()
      Vector= class('Vector')
      function Vector.initialize(a,x,y,z) a.x, a.y, a.z = x,y,z end
      function Vector.__tostring(a) return a.class.name .. '[' .. a.x .. ',' .. a.y .. ',' .. a.z .. ']' end
      function Vector.__eq(a,b)     return a.x==b.x and a.y==b.y and a.z==b.z end
      function Vector.__lt(a,b)     return a() < b() end
      function Vector.__le(a,b)     return a() <= b() end
      function Vector.__add(a,b)    return a.class:new(a.x+b.x, a.y+b.y ,a.z+b.z) end
      function Vector.__sub(a,b)    return a.class:new(a.x-b.x, a.y-b.y, a.z-b.z) end
      function Vector.__div(a,s)    return a.class:new(a.x/s, a.y/s, a.z/s) end
      function Vector.__unm(a)      return a.class:new(-a.x, -a.y, -a.z) end
      function Vector.__concat(a,b) return a.x*b.x+a.y*b.y+a.z*b.z end
      function Vector.__call(a)     return math.sqrt(a.x*a.x+a.y*a.y+a.z*a.z) end
      function Vector.__pow(a,b)
        return a.class:new(a.y*b.z-a.z*b.y,a.z*b.x-a.x*b.z,a.x*b.y-a.y*b.x)
      end
      function Vector.__mul(a,b)
        if type(b)=="number" then return a.class:new(a.x*b, a.y*b, a.z*b) end
        if type(a)=="number" then return b.class:new(a*b.x, a*b.y, a*b.z) end
      end
      Vector.__metatable = "metatable of a vector"
      Vector.__mode = "k"

      v = Vector:new(1,2,3)
      w = Vector:new(2,4,6)
    end)

    it('implements __tostring', function()
      assert.equal(tostring(v), "Vector[1,2,3]")
    end)

    it('implements __eq', function()
      assert.equal(v, v)
    end)

    it('implements __lt', function()
      assert.is_true(v < w)
    end)

    it('implements __le', function()
      assert.is_true(v <= w)
    end)

    it('implements __add', function()
      assert.equal(v+w, Vector(3,6,9))
    end)

    it('implements __sub', function()
      assert.equal(w-v, Vector(1,2,3))
    end)

    it('implements __div', function()
      assert.equal(w/2, Vector(1,2,3))
    end)

    it('implements __concat', function()
      assert.equal(v..w, 28)
    end)

    it('implements __call', function()
      assert.equal(v(), math.sqrt(14))
    end)

    it('implements __pow', function()
      assert.equal(v^w, Vector(0,0,0))
    end)

    it('implements __mul', function()
      assert.equal(4*v, Vector(4,8,12))
    end)

    it('implements __metatable', function()
      assert.equal("metatable of a vector", getmetatable(v))
    end)

    it('implements __mode', function()
      v[{}] = true
      collectgarbage()
      for k in pairs(v) do assert.not_table(k) end
    end)

    it('instance should have a table __index if not overrided', function()
      assert.is_true(type(v.class.__instanceDict.__index) == 'table')
    end)

    --[[
    it('implements __index', function()
      assert.equal(b[1], 3)
    end)
    --]]

    describe('Inherited Metamethods', function()
      local Vector2, v2, w2
      before_each(function()
        Vector2= class('Vector2', Vector)
        function Vector2:initialize(x,y,z) Vector.initialize(self,x,y,z) end

        v2 = Vector2:new(1,2,3)
        w2 = Vector2:new(2,4,6)
      end)

      it('implements __tostring', function()
        assert.equal(tostring(v2), "Vector2[1,2,3]")
      end)

      it('implements __eq', function()
        assert.equal(v2, v2)
      end)

      it('implements __lt', function()
        assert.is_true(v2 < w2)
      end)

      it('implements __le', function()
        assert.is_true(v2 <= w2)
      end)

      it('implements __add', function()
        assert.equal(v2+w2, Vector2(3,6,9))
      end)

      it('implements __sub', function()
        assert.equal(w2-v2, Vector2(1,2,3))
      end)

      it('implements __div', function()
        assert.equal(w2/2, Vector2(1,2,3))
      end)

      it('implements __concat', function()
        assert.equal(v2..w2, 28)
      end)

      it('implements __call', function()
        assert.equal(v2(), math.sqrt(14))
      end)

      it('implements __pow', function()
        assert.equal(v2^w2, Vector2(0,0,0))
      end)

      it('implements __mul', function()
        assert.equal(4*v2, Vector2(4,8,12))
      end)

      it('implements __metatable', function()
        assert.equal("metatable of a vector", getmetatable(v2))
      end)

      it('implements __mode', function()
        v2[{}] = true
        collectgarbage()
        for k in pairs(v2) do assert.not_table(k) end
      end)

      it('instance should also have a table __index if not overrided', function()
        assert.is_true(type(v2.class.__instanceDict.__index) == 'table')
      end)

      it('allows inheriting further', function()
        local Vector3 = class('Vector3', Vector2)
        local v3 = Vector3(1,2,3)
        local w3 = Vector3(3,4,5)
        assert.equal(v3+w3, Vector3(4,6,8))
      end)

      describe('Updates', function()
        it('overrides __add', function()
          Vector2.__add = function(a, b) return Vector.__add(a, b)/2 end
          assert.equal(v2+w2, Vector2(1.5,3,4.5))
        end)

        it('updates __add', function()
          Vector.__add = Vector.__sub
          assert.equal(v2+w2, Vector2(-1,-2,-3))
        end)

        it('does not update __add after overriding', function()
          Vector2.__add = function(a, b) return Vector.__add(a, b)/2 end
          Vector.__add = Vector.__sub
          assert.equal(v2+w2, Vector2(-0.5,-1,-1.5))
        end)

        it('reverts __add override', function()
          Vector2.__add = function(a, b) return Vector.__add(a, b)/2 end
          Vector2.__add = nil
          assert.equal(v2+w2, Vector2(3,6,9))
        end)
      end)
    end)
  end)

  describe('Custom __index and __newindex', function()
    describe('Tables', function()
      local Proxy, fallback, p
      before_each(function()
        Proxy = class('Proxy')
        fallback = {foo = 'bar', common = 'fallback'}
        Proxy.__index = fallback
        Proxy.__newindex = fallback
        Proxy.common = 'class'
        p = Proxy()
      end)

      it('uses __index', function()
        assert.equal(p.foo, 'bar')
      end)

      it('does not use __index when field exists in class', function()
        assert.equal(p.common, 'class')
      end)

      it('uses __newindex', function()
        p.key = 'value'
        assert.equal(fallback.key, 'value')
      end)

      it('uses __newindex when field exists in class', function()
        p.common = 'value'
        assert.equal(p.common, 'class')
        assert.equal(Proxy.common, 'class')
        assert.equal(fallback.common, 'value')
      end)
    end)

    describe('Functions', function()
      local Namespace, Rectangle, r
      before_each(function()
        Namespace = class('Namespace')
        function Namespace:__index(name)
          local getter = self.class[name.."Getter"]
          if getter then return getter(self) end
        end
        function Namespace:__newindex(name, value)
          local setter = self.class[name.."Setter"]
          if setter then setter(self, value) else rawset(self, name, value) end
        end
        Rectangle = class('Rectangle', Namespace)
        function Rectangle:initialize(x, y, scale)
          self._scale, self.x, self.y = 1, x, y
          self.scale = scale
        end
        function Rectangle:scaleGetter() return self._scale end
        function Rectangle:scaleSetter(v)
          self.x = self.x*v/self._scale
          self.y = self.y*v/self._scale
          self._scale = v
        end
        function Rectangle:areaGetter() return self.x * self.y end
        r = Rectangle(3, 4, 2)
      end)

      it('uses setter', function()
        assert.equal(r.x, 6)
        assert.equal(r.y, 8)
        r.scale = 3
        assert.equal(r.x, 9)
        assert.equal(r.y, 12)
      end)

      it('uses getters', function()
        assert.equal(r.scale, 2)
        assert.equal(r.area, 48)
      end)

      it('updates inherited __index', function()
        function Namespace.__index() return 42 end
        assert.equal(r.area, 42)
        function Rectangle.__index() return 24 end
        assert.equal(r.area, 24)
        function Namespace.__index() return 96 end
        assert.equal(r.area, 24)
        Rectangle.__index = nil
        assert.equal(r.area, 96)
      end)
    end)
  end)

  describe('Default Metamethods', function()

    local Peter, peter

    before_each(function()
      Peter = class('Peter')
      peter = Peter()
    end)

    describe('A Class', function()
      it('has a call metamethod properly set', function()
        assert.is_true(peter:isInstanceOf(Peter))
      end)
      it('has a tostring metamethod properly set', function()
        assert.equal(tostring(Peter), 'class Peter')
      end)
    end)

    describe('An instance', function()
      it('has a tostring metamethod, returning a different result from Object.__tostring', function()
        assert.equal(tostring(peter), 'instance of class Peter')
      end)
    end)
  end)

end)
end

-- mixins_spec.lua
do
describe('A Mixin', function()

  local Mixin1, Mixin2, Class1, Class2

  before_each(function()
    Mixin1, Mixin2 = {},{}

    function Mixin1:included(theClass) theClass.includesMixin1 = true end
    function Mixin1:foo() return 'foo' end
    function Mixin1:bar() return 'bar' end
    Mixin1.static = {}
    Mixin1.static.bazzz = function() return 'bazzz' end


    function Mixin2:baz() return 'baz' end

    Class1 = class('Class1'):include(Mixin1, Mixin2)
    function Class1:foo() return 'foo1' end

    Class2 = class('Class2', Class1)
    function Class2:bar2() return 'bar2' end
  end)

  it('invokes the "included" method when included', function()
    assert.is_true(Class1.includesMixin1)
  end)

  it('has all its functions (except "included") copied to its target class', function()
    assert.equal(Class1:bar(), 'bar')
    assert.is_nil(Class1.included)
  end)

  it('makes its functions available to subclasses', function()
    assert.equal(Class2:baz(), 'baz')
  end)

  it('allows overriding of methods in the same class', function()
    assert.equal(Class2:foo(), 'foo1')
  end)

  it('allows overriding of methods on subclasses', function()
    assert.equal(Class2:bar2(), 'bar2')
  end)

  it('makes new static methods available in classes', function()
    assert.equal(Class1:bazzz(), 'bazzz')
    assert.equal(Class2:bazzz(), 'bazzz')
  end)

end)
end

-- metamethods_lua_5_2.lua: only __len, since Luau's pairs and ipairs ignore __pairs and
-- __ipairs. Luau's own __iter and __idiv take their place
do
describe('Luau Metamethods', function()
  local Vector, v
  before_each(function()
    Vector= class('Vector')
    function Vector.initialize(a,x,y,z) a.x, a.y, a.z = x,y,z end
    function Vector.__eq(a,b)     return a.x==b.x and a.y==b.y and a.z==b.z end
    function Vector.__len(a)    return 3 end
    function Vector.__idiv(a,s) return a.class:new(a.x//s, a.y//s, a.z//s) end
    function Vector.__iter(a)
      local i = 0
      local t = {a.x,a.y,a.z}
      return function()
        i = i + 1
        if t[i] then return i, t[i] end
      end
    end
    v = Vector:new(1,2,3)
  end)

  it('implements __len', function()
    assert.equal(#v, 3)
  end)

  it('implements __idiv', function()
    assert.equal(v//2, Vector(0,1,1))
  end)

  it('implements __iter',function()
    local output = {}
    for _,i in v do
      output[#output+1] = i
    end
    assert.are.same(output,{1,2,3})
  end)

  describe('Inherited Metamethods', function()
    local Vector2, v2
    before_each(function()
      Vector2= class('Vector2', Vector)
      function Vector2:initialize(x,y,z) Vector.initialize(self,x,y,z) end
      v2 = Vector2:new(1,2,3)
    end)

    it('implements __len', function()
      assert.equal(#v2, 3)
    end)

    it('implements __idiv', function()
      assert.equal(v2//2, Vector2(0,1,1))
    end)

    it('implements __iter',function()
      local output = {}
      for _,i in v2 do
        output[#output+1] = i
      end
      assert.are.same(output,{1,2,3})
    end)
  end)
end)
end
