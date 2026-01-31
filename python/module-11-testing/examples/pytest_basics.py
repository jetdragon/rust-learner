# pytest 测试框架演示

"""
本文件演示pytest测试框架的使用和最佳实践。
"""


def test_example():
    """简单的测试示例"""
    assert 1 + 1 == 2
    assert "hello".upper() == "HELLO"


# ========== 1. 断言 (Assertions) ==========
def test_assertions():
    """各种断言方式"""
    # 相等
    assert 1 + 1 == 2

    # 不等
    assert 1 != 2

    # 真/假
    assert True
    assert not False

    # 包含
    assert 2 in [1, 2, 3]

    # 类型
    assert isinstance("hello", str)

    # 异常
    with pytest.raises(ValueError):
        int("invalid")


# ========== 2. 参数化测试 ==========
import pytest


@pytest.mark.parametrize("a,b,expected", [
    (1, 2, 3),
    (0, 0, 0),
    (-1, 1, 0),
    (100, 200, 300),
])
def test_add(a, b, expected):
    """参数化测试 - 测试多组数据"""
    assert a + b == expected


# ========== 3. Fixture (测试夹具) ==========
@pytest.fixture
def sample_data():
    """准备测试数据"""
    return {"name": "Alice", "age": 30}


@pytest.fixture
def temp_file(tmp_path):
    """临时文件 fixture"""
    file_path = tmp_path / "test.txt"
    file_path.write_text("测试内容")
    return file_path


def test_with_fixture(sample_data):
    """使用 fixture"""
    assert sample_data["name"] == "Alice"
    assert sample_data["age"] == 30


def test_with_temp_file(temp_file):
    """使用临时文件"""
    content = temp_file.read_text()
    assert content == "测试内容"


# ========== 4. Fixture 作用域 ==========
@pytest.fixture(scope="module")
def module_resource():
    """模块级 fixture（只创建一次）"""
    print("创建模块级资源")
    resource = {"value": 100}
    yield resource
    print("清理模块级资源")


@pytest.fixture(scope="function")
def function_resource():
    """函数级 fixture（每个测试都创建）"""
    print("创建函数级资源")
    return {"value": 1}


def test_scope_1(module_resource, function_resource):
    """测试1"""
    assert module_resource["value"] == 100
    assert function_resource["value"] == 1


def test_scope_2(module_resource, function_resource):
    """测试2"""
    assert module_resource["value"] == 100
    assert function_resource["value"] == 1


# ========== 5. 测试类 ==========
class TestCalculator:
    """测试类（组织相关测试）"""

    @pytest.fixture(autouse=True)
    def setup(self):
        """每个测试前执行"""
        print("setup: 初始化计算器")

    def test_add(self):
        """测试加法"""
        assert 1 + 1 == 2

    def test_subtract(self):
        """测试减法"""
        assert 5 - 3 == 2

    def test_multiply(self):
        """测试乘法"""
        assert 3 * 4 == 12


# ========== 6. 跳过和标记测试 ==========
@pytest.mark.skip(reason="暂未实现")
def test_not_ready():
    """跳过测试"""
    assert False


@pytest.mark.skipif(
    pytest.__version__ < "7.0",
    reason="pytest 7.0+ 特性"
)
def test_version_specific():
    """条件跳过"""
    assert True


@pytest.mark.slow
def test_slow_operation():
    """标记为慢速测试"""
    import time
    time.sleep(0.1)
    assert True


# ========== 7. 异常测试 ==========
def test_exception():
    """测试异常"""
    with pytest.raises(ValueError) as exc_info:
        raise ValueError("错误信息")
    assert str(exc_info.value) == "错误信息"


def test_exception_match():
    """匹配异常信息"""
    with pytest.raises(ValueError, match="错误"):
        raise ValueError("错误信息")


# ========== 8. 警告测试 ==========
import warnings


def test_warning():
    """测试警告"""
    warnings.warn("这是一个警告", UserWarning)
    assert True


# ========== 9. 临时目录和文件 ==========
def test_tmp_path(tmp_path):
    """使用临时目录"""
    # 创建文件
    file_path = tmp_path / "test.txt"
    file_path.write_text("Hello")

    # 验证
    assert file_path.exists()
    assert file_path.read_text() == "Hello"


def test_tmpdir(tmpdir):
    """使用 tmpdir fixture（旧版本）"""
    file_path = tmpdir.join("test.txt")
    file_path.write("Hello")
    assert file_path.read() == "Hello"


# ========== 10. Monkeypatch ==========
def test_monkeypatch(monkeypatch):
    """monkeypatch - 临时替换对象"""
    # 替换函数
    def mock_function():
        return "mocked"

    monkeypatch.setattr("some_module.some_function", mock_function)

    # 临时修改环境变量
    monkeypatch.setenv("TEST_VAR", "test_value")
    import os
    assert os.getenv("TEST_VAR") == "test_value"


# ========== 11. Capsys - 捕获输出 ==========
def test_capsys(capsys):
    """捕获 print 输出"""
    print("Hello")
    print("World")

    captured = capsys.readouterr()
    assert "Hello" in captured.out
    assert "World" in captured.out


# ========== 12. 测试覆盖率 ==========
def test_coverage_example():
    """覆盖率示例"""
    # 测试所有分支
    def is_positive(x):
        if x > 0:
            return True
        else:
            return False

    assert is_positive(1) is True
    assert is_positive(-1) is False
    assert is_positive(0) is False


if __name__ == "__main__":
    # 可以直接运行测试
    # pytest this_file.py -v
    print("使用 pytest 运行此文件:")
    print("  pytest test_example.py -v")
    print("  pytest test_example.py -v -m slow  # 只运行慢速测试")
    print("  pytest test_example.py -v -k 'test_add'  # 只运行匹配的测试")
    print("  pytest test_example.py -v --cov=.  # 测试覆盖率")
