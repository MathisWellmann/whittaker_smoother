# Whittaker Smoother
Aka Whittaker-Henderson, Whittaker-Eilers Smoother is known as the perfect smoother;
is a discrete-time version of spline smoothing for equally spaced data.
It minimizes the functional

$$\sum_{i=0}^n (z_i - y_i)^2 + \lambda \sum_{i=0}^n (\delta ^2 z)_i ^1  $$

where y are the data, z is the smoothed function, and \delta^2
z is the pth derivative of z_i, which is evaluated numerically.
A penalty is imposed on nonsmooth functions, with higher values of \lambda increasing the penalty and leading to a smoother output.

The smoothed output can be obtained by solving the linear system
$$x = (W + \lambda * D^T D )^-1 W y $$
Where W is the weight matrix (Identity matrix in practice).

### Examples
![wood_2](img/whittaker_on_wood_lambda_20000_order_2.png)
![wood_3](img/whittaker_on_wood_lambda_20000_order_3.png)

### Comparison to Moving Averages and Convolution Kernels
Compared to a moving average smoother, this method does not suffer from a group-delay.
Compared to a convolution kernel such as the savitzky-golay filter, 
the values at the edge are well defined. The savitzky-golay filter does have a nice flat passband,
but suffers from unsatisfactory high-frequency noise, which is not sufficiently suppressed. 
This is a particular problem when the derivative of the data is of importance.

### Further Reading:
See the [papers](./papers/) folder for two papers showing additional details of the method.

This implementation was inspired by [A python implementation](https://github.com/mhvwerts/whittaker-eilers-smoother).


