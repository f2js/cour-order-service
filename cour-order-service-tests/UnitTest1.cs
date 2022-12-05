using cour_order_service.Controllers;

namespace cour_order_service_tests;
public class Tests
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void Test1()
    {
        var control = new WeatherForecastController(null);
        var res = control.Get();
        Assert.That(res.Count(), Is.EqualTo(5));
    }
}