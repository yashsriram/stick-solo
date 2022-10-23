x_size = size(x);
x_size = x_size(2);
y_size = size(y);
y_size = y_size(2);
A = reshape(a, [y_size, x_size]);
B = reshape(b, [y_size, x_size]);
D = reshape(d, [y_size, x_size]);

surf(x, y, D)
xlabel('input x')
ylabel('input y')
zlabel('distance of holding goal from origin')

% quiver(x, y, A, B)
% xlabel('input x')
% ylabel('input y')
