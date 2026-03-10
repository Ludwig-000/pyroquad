import pyroquad as pq

def get_closest_point(point, rect_pos, rect_size):
    cx = max(rect_pos.x, min(point.x, rect_pos.x + rect_size.x))
    cy = max(rect_pos.y, min(point.y, rect_pos.y + rect_size.y))
    return pq.Vec2(cx, cy)

def run_splash_screen():
    # Setup 3D camera
    cam3d = pq.Camera3D(position=pq.Vec3(0, 0, -10))
    # Setup Cube
    cube = pq.Cube(position=pq.Vec3(0, 0, 0), scale= pq.Vec3.splat(5))

    while not pq.is_quit_requested():
        dt = pq.get_delta_time()
        
        
        
        # 2D rendering
        pq.set_default_camera()
        
        sw = pq.screen_width()
        sh = pq.screen_height()
        
        title = "PONG"
        subtitle = "Press SPACE to start"
        
        # Simple text positioning (can be improved with text measurements if available)
        pq.draw_text(title, sw / 2 - 100, sh / 2 - 50, pq.Color.WHITE, 100)
        pq.draw_text(subtitle, sw / 2 - 130, sh / 2 + 50, pq.Color.DARKGRAY, 30)
        

        # 3D rendering
        pq.Camera3D.set_camera(cam3d)
        cube.rot += pq.Vec3(1.0, 1.5, 0.5) * dt
        pq.draw_all_objects()

        keys = pq.get_keys_down()
        if pq.KeyCode.Space in keys:
            cube.set_draw_each_frame(False) # Turn off cube for main game
            break
        pq.next_frame()

def main():
    pq.activate_engine(pq.Config())
    
    # Run the splash screen first
    run_splash_screen()
    
    # Paddle settings
    paddle_size = pq.Vec2(20, 100)
    paddle_speed = 400
    
    player_pos = pq.Vec2(50, pq.screen_height() / 2 - paddle_size.y / 2)
    ai_pos = pq.Vec2(pq.screen_width() - 50 - paddle_size.x, pq.screen_height() / 2 - paddle_size.y / 2)
    
    # Ball settings
    ball_pos = pq.Vec2(pq.screen_width() / 2, pq.screen_height() / 2)
    ball_vel = pq.Vec2(300, 300)
    ball_r = 10
    
    # Game settings
    score_player = 0
    score_ai = 0
    
    while not pq.is_quit_requested():
        dt = pq.get_delta_time()
        sw = pq.screen_width()
        sh = pq.screen_height()
        
        # Determine valid paddle bounds
        player_pos = pq.Vec2(player_pos.x, max(0, min(sh - paddle_size.y, player_pos.y)))
        ai_pos = pq.Vec2(sw - 50 - paddle_size.x, max(0, min(sh - paddle_size.y, ai_pos.y)))
        
        # -- Player input --
        keys = pq.get_keys_down()
        if pq.KeyCode.W in keys:
            player_pos += pq.Vec2(0, -paddle_speed * dt)
        if pq.KeyCode.S in keys:
            player_pos += pq.Vec2(0, paddle_speed * dt)
            
        player_pos = pq.Vec2(player_pos.x, max(0, min(sh - paddle_size.y, player_pos.y)))
        
        # -- AI logic --
        ai_center = ai_pos.y + paddle_size.y / 2
        # Move towards the ball y position
        if ai_center < ball_pos.y - 10:
            ai_pos += pq.Vec2(0, paddle_speed * 0.85 * dt)
        elif ai_center > ball_pos.y + 10:
            ai_pos += pq.Vec2(0, -paddle_speed * 0.85 * dt)
            
        ai_pos = pq.Vec2(ai_pos.x, max(0, min(sh - paddle_size.y, ai_pos.y)))
        
        # -- Ball update --
        ball_pos += ball_vel * dt
        
        # Top and bottom bounce
        if ball_pos.y - ball_r < 0:
            ball_pos = pq.Vec2(ball_pos.x, ball_r)
            ball_vel = pq.Vec2(ball_vel.x, abs(ball_vel.y))
        elif ball_pos.y + ball_r > sh:
            ball_pos = pq.Vec2(ball_pos.x, sh - ball_r)
            ball_vel = pq.Vec2(ball_vel.x, -abs(ball_vel.y))
            
        # Paddle collisions
        # Player (left)
        if ball_vel.x < 0:
            closest_p = get_closest_point(ball_pos, player_pos, paddle_size)
            dist_v = ball_pos + pq.Vec2(-closest_p.x, -closest_p.y)
            if (dist_v.x**2 + dist_v.y**2) <= ball_r**2:
                ball_vel = pq.Vec2(abs(ball_vel.x) * 1.05, ball_vel.y)
                ball_pos = pq.Vec2(player_pos.x + paddle_size.x + ball_r, ball_pos.y)
                hit_factor = (ball_pos.y - (player_pos.y + paddle_size.y/2)) / (paddle_size.y/2)
                ball_vel = pq.Vec2(ball_vel.x, ball_vel.y * 0.5 + hit_factor * 200)
            
        # AI (right)
        if ball_vel.x > 0:
            closest_p = get_closest_point(ball_pos, ai_pos, paddle_size)
            dist_v = ball_pos + pq.Vec2(-closest_p.x, -closest_p.y)
            if (dist_v.x**2 + dist_v.y**2) <= ball_r**2:
                ball_vel = pq.Vec2(-abs(ball_vel.x) * 1.05, ball_vel.y)
                ball_pos = pq.Vec2(ai_pos.x - ball_r, ball_pos.y)
                hit_factor = (ball_pos.y - (ai_pos.y + paddle_size.y/2)) / (paddle_size.y/2)
                ball_vel = pq.Vec2(ball_vel.x, ball_vel.y * 0.5 + hit_factor * 200)
            
        # Scoring
        if ball_pos.x + ball_r < 0:
            score_ai += 1
            ball_pos = pq.Vec2(sw / 2, sh / 2)
            ball_vel = pq.Vec2(300, 300)
        elif ball_pos.x - ball_r > sw:
            score_player += 1
            ball_pos = pq.Vec2(sw / 2, sh / 2)
            ball_vel = pq.Vec2(-300, -300)

        # -- Rendering --
        pq.clear_background(pq.Color.BLACK)
        
        # Center line
        cx = sw / 2
        for y in range(0, int(sh), 40):
            pq.draw_rectangle(cx - 2, y + 10, 4, 20, pq.Color.DARKGRAY)
        
        # Paddles
        pq.draw_rectangle(player_pos.x, player_pos.y, paddle_size.x, paddle_size.y, pq.Color.WHITE)
        pq.draw_rectangle(ai_pos.x, ai_pos.y, paddle_size.x, paddle_size.y, pq.Color.WHITE)
        
        # Ball
        pq.draw_circle(ball_pos.x, ball_pos.y, ball_r, pq.Color.WHITE)
        
        # Score Text
        pq.draw_text(str(score_player), cx - 100, 50, color=pq.Color.WHITE, font_size=50)
        pq.draw_text(str(score_ai), cx + 50, 50, color=pq.Color.WHITE, font_size=50)
        
        pq.next_frame()

if __name__ == "__main__":
    main()
