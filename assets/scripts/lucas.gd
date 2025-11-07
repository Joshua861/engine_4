extends Polygon2D
var sizes = [4,5,5,5,5,5,5,5,4.8,4.4,4.0 ,3.6 ,3.4 ,3.2,3.0,2.8, 2.7, 2.3,1.5,1,1,1,1,0.5]
var legPoints = [[]]

var Bodypoints = []
var VertexPoints = []
var node = preload("res://leg.tscn")
var radius = 2
var time = 0
var PPoint = []
var Target
var spinning = false

var timespining = 0
@export var move_speed: float = 100
func _ready() -> void:
	legPoints.append([])
	legPoints.append([])
	legPoints.append([])
	legPoints.append([])
	for i in range(0, legPoints.size()-1):
		PPoint.append(Vector2(0,0))
		var instance = node.instantiate()

		instance.name = str(i)
		legPoints[i].append(Vector2(0,0))
		legPoints[i].append(Vector2(0,0))
		legPoints[i].append(Vector2(0,0))
		instance.points = legPoints[i]
		add_child(instance)
	for i in range(0, sizes.size()):
		Bodypoints.append(Vector2(0.0,0.0))

	for i in range(0, Bodypoints.size()):
		var angle = atan2(Bodypoints[i].y - Bodypoints[i-1].y, Bodypoints[i].x - Bodypoints[i-1].x)		#if angle >
		if i != 0:
			Bodypoints[i].x = Bodypoints[i-1].x +sin(angle) *radius
			Bodypoints[i].y = Bodypoints[i-1].y + cos(angle) * radius
			VertexPoints.append(Vector2(sin(angle+PI/2) +Bodypoints[i].x, sizes[i] *cos(angle+PI/2)+Bodypoints[i].y))
			VertexPoints.append(Vector2(sin(angle-PI/2) +Bodypoints[i].x, sizes[i] *cos(angle-PI/2)+Bodypoints[i].y))
		else:
			VertexPoints.append(Vector2(sin(angle+PI/4) +Bodypoints[i].x, sizes[i] *cos(angle+PI/4)+Bodypoints[i].y))
			VertexPoints.append(Vector2(sin(angle-PI/4) +Bodypoints[i].x, sizes[i] *cos(angle-PI/4)+Bodypoints[i].y))

			VertexPoints.append(Vector2(sin(angle+PI/2) +Bodypoints[i].x, sizes[i] *cos(angle+PI/2)+Bodypoints[i].y))
			VertexPoints.append(Vector2(sin(angle-PI/2) +Bodypoints[i].x, sizes[i] *cos(angle-PI/2)+Bodypoints[i].y))

	var NewVertexPoints = []
	for i in range(0, VertexPoints.size()/2):
		NewVertexPoints.append(VertexPoints[2*i])
	for i in range(0, VertexPoints.size()/2):

		NewVertexPoints.append(VertexPoints[2*i+1])
	polygon = NewVertexPoints

func _physics_process(delta):

	var mouse_pos = get_global_mouse_position()
	var direction = (mouse_pos - global_position).normalized()
	if Target == null:
		var a
		if spinning == false:
			a = atan2(mouse_pos.x - Bodypoints[0].x, mouse_pos.y - Bodypoints[0].y)
		else:
			a = atan2(Bodypoints[0].x - Bodypoints[1].x, Bodypoints[0].y - Bodypoints[1].y)

		var distance = sqrt( pow(mouse_pos.x - Bodypoints[0].x, 2) + pow(mouse_pos.y - Bodypoints[0].y, 2))
		if distance > 10:
			Bodypoints[0].x += 0.9*sin(a)
			Bodypoints[0].y += 0.9*cos(a)
	else:
		var a = atan2(Target.x - Bodypoints[0].x, Target.y - Bodypoints[0].y)
		var distance = sqrt( pow(Target.x - Bodypoints[0].x, 2) + pow(Target.y - Bodypoints[0].y, 2))
		if distance > 1:
			Bodypoints[0].x += 2*sin(a)
			Bodypoints[0].y += 2*cos(a)
			get_node("tounge").points[0] = Bodypoints[0]
		else:
			Target = null
			get_node("tounge").queue_free()

	var NewVertexPoints = []
	for i in range(0, Bodypoints.size()):
		var angle = atan2(Bodypoints[i].x - Bodypoints[i-1].x, Bodypoints[i].y - Bodypoints[i-1].y)
		var Pangle = atan2(Bodypoints[i-1].x - Bodypoints[i-2].x, Bodypoints[i-1].y - Bodypoints[i-2].y)
		if Pangle - angle > 3 :
			pass
			#angle -= 0.01
		if Pangle - angle < -3:
			pass
			#angle += 0.01
			#angle = Pangle
		if i != 0:
			Bodypoints[i].x = Bodypoints[i-1].x + (sin(angle) *radius)
			Bodypoints[i].y = Bodypoints[i-1].y + (cos(angle) * radius)
		else:
			if spinning == false and Target == null:
				angle = atan2(Bodypoints[i].x - mouse_pos.x, Bodypoints[i].y - mouse_pos.y)
			else:
				angle = atan2(Bodypoints[1].x - Bodypoints[0].x, Bodypoints[1].y - Bodypoints[0].y)
			VertexPoints[0] = (Vector2(sizes[0] *sin(angle+5*PI/6) +Bodypoints[0].x, sizes[0] *cos(angle+5*PI/6)+Bodypoints[0].y))
			VertexPoints[1]= (Vector2(sizes[0] *sin(angle-5*PI/6) +Bodypoints[0].x, sizes[0] *cos(angle-5*PI/6)+Bodypoints[0].y))
			var node = str(0)

			#get_node(node).position = VertexPoints[0]
			#node = str(1)
			#get_node(node).position = VertexPoints[1]
			NewVertexPoints.append(VertexPoints[1])
			NewVertexPoints.append(VertexPoints[0])
		VertexPoints[2*i+2] = (Vector2(sizes[i] *sin(angle+PI/2) +Bodypoints[i].x, sizes[i] *cos(angle+PI/2)+Bodypoints[i].y))
		VertexPoints[2*i+3] = (Vector2(sizes[i] *sin(angle-PI/2) +Bodypoints[i].x, sizes[i] *cos(angle-PI/2)+Bodypoints[i].y))
		draw
		var node = str(2*i+2)
		NewVertexPoints.append(VertexPoints[2*i +2 ])
		leg(10, 4, 0)
		leg(30,12,1)
		leg(11,3,2)

		leg(31,12,3)
	for i in range(0, Bodypoints.size()):

		NewVertexPoints.append(VertexPoints[VertexPoints.size()- 2* i -1])
	#NewVertexPoints.append(VertexPoints[1])
	polygon = NewVertexPoints
	get_node("Camera2D").position = Bodypoints[0]
	var node = "border"
	get_node(node).points = NewVertexPoints

func _draw():
	draw_circle(Vector2(VertexPoints[3].x,VertexPoints[3].y), 2, Color(1,1,1), true)
	draw_circle(Vector2(VertexPoints[2].x,VertexPoints[2].y), 2, Color(1,1,1), true)
	draw_circle(Vector2(VertexPoints[3].x,VertexPoints[3].y), 1, Color(0,0,0), true)
	draw_circle(Vector2(VertexPoints[2].x,VertexPoints[2].y), 1, Color(0,0,0), true)

func leg(anchor,Segment, leg):
	var sign
	var legSize = 1.5
	if anchor % 2 == 0:
		sign = 1
	else:
		sign = -1

	var a = atan2(Bodypoints[Segment].x - Bodypoints[Segment-1].x, Bodypoints[Segment].y - Bodypoints[Segment-1].y)
	var point = Vector2(0,0)
	point.x = 2*sizes[Segment] *sin(a+ sign * PI/2) +Bodypoints[Segment].x
	point.y = 2* sizes[Segment] *cos(a+ sign *PI/2)+Bodypoints[Segment].y
	time += 1
	if  Target == null and sqrt(pow(PPoint[leg].x -point.x, 2) + pow(PPoint[leg].y -point.y, 2) > 150):#3*sizes[Segment] * legPoints[leg].size()):
		PPoint[leg] = point
	else:
		point = PPoint[leg]
	legPoints[leg][0] = VertexPoints[anchor]
	for N in range(0, 5):
		legPoints[leg][legPoints[leg].size()-1] = point
		for i in range(1, legPoints[leg].size()):
			var y = legPoints[leg].size()-1 - i
			var angle = atan2(legPoints[leg][y].x - legPoints[leg][y+1].x, legPoints[leg][y].y - legPoints[leg][y+1].y)
			legPoints[leg][y].x = legPoints[leg][y+1].x +sin(angle)*radius*legSize
			legPoints[leg][y].y = legPoints[leg][y+1].y +cos(angle)*radius*legSize
		legPoints[leg][0] = VertexPoints[anchor]
		for y in range(1, legPoints[leg].size()):
			var angle = atan2(legPoints[leg][y].x - legPoints[leg][y-1].x, legPoints[leg][y].y - legPoints[leg][y-1].y)
			legPoints[leg][y].x = legPoints[leg][y-1].x +sin(angle)*radius*legSize
			legPoints[leg][y].y = legPoints[leg][y-1].y +cos(angle)*radius*legSize
	var nodename = str(leg)
	get_node(nodename).points= legPoints[leg]

func _unhandled_input(event):
	if event.is_action_pressed("shoot") and Target == null:
		var toungePoints = []
		var mouse_pos = get_global_mouse_position()
		var angle = atan2(Bodypoints[0].x - mouse_pos.x, Bodypoints[0].y - mouse_pos.y)
		toungePoints.append(Bodypoints[0])


		var ray = RayCast2D.new()
		ray.collide_with_areas = true
		ray.position = Bodypoints[0]
		ray.target_position = Vector2(Bodypoints[0].x  -100*sin(angle) -ray.position.x, Bodypoints[0].y -100*cos(angle) -ray.position.y)
		add_child(ray)
		ray.force_raycast_update()
		if ray.is_colliding() == true:
			if ray.get_collider().is_in_group("predator"):
				toungePoints.append(Vector2(ray.get_collision_point()))
				ray.get_collider().linear_velocity = Vector2(20*sin  (angle), 20*cos(angle))
				Target = Vector2(ray.get_collision_point())
		else:
			toungePoints.append(Vector2( Bodypoints[0].x  -100*sin(angle), Bodypoints[0].y -100*cos(angle)))
			Target = Vector2( Bodypoints[0].x  -100*sin(angle), Bodypoints[0].y -100*cos(angle))


		var instance = node.instantiate()
		instance.modulate = Color(0.939, 0.62,0.703)
		instance.width = 2.0
		instance.z_index = -2
		instance.points = toungePoints
		instance.name = "tounge"
		add_child(instance)

	if event.is_action_pressed("attack"):
		modulate = Color(1,0,0)
		var mouth = get_node("mouth")
		mouth.monitoring = true
		mouth.position = Bodypoints[0]
		for i in range(0, 10):
			await get_tree().process_frame

		var bodies = mouth.get_overlapping_bodies()
		for i in range(0, bodies.size()):
			if bodies[i].is_in_group("predator") == true:
				bodies[i].damaged(2)
				var mouse_pos = get_global_mouse_position()
				var angle = atan2(Bodypoints[0].x - bodies[i].position.x, bodies[0].position.y - bodies[i].position.y)
				bodies[i].linear_velocity += Vector2( -80*sin  (angle), -80*cos(angle))
				#Bodypoints[0] =  Vector2(Bodypoints[0].x +10*sin  (angle),Bodypoints[0].y +10*cos(angle))
				pass
		mouth.monitoring = false
		modulate = Color(1, 1,1)
	if event.is_action("spin"):
		timespining += 0.01
		spinning = true

		for i in range(0, Bodypoints.size()):#Bodypoints.size()):
			var x= Bodypoints[i].x
			var y = Bodypoints[i].y
			var angle = log(timespining+1)/5
			if angle > 0.2:
				angle = 0.2

			var XN = x - Bodypoints[Bodypoints.size()/2].x
			var YN = y -  Bodypoints[Bodypoints.size()/2].y

			var XNN = XN* cos(angle) - YN * sin(angle)
			var YNN =  XN * sin(angle) + YN * cos(angle)

			Bodypoints[i].x =  XNN + Bodypoints[Bodypoints.size()/2].x
			Bodypoints[i].y = YNN + Bodypoints[Bodypoints.size()/2].y
			#VertexPoints[2*i].x =
	if event.is_action_released("spin"):
		timespining = 0
		spinning = false
